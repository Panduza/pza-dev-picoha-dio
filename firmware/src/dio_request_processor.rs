// Print debug support
use crate::api_dio_utils;
#[cfg(any(feature = "uart0_debug"))]
use crate::uart_debug::uart_debug_print;
use crate::{
    api_dio::{PicohaDioAnswer, PicohaDioRequest},
    print_debug_message,
};
use core::fmt::Write;

use embedded_hal::digital::OutputPin;
use rp2040_hal::gpio::new_pin;
// Message deserialization support
use femtopb::Message;

use rp2040_hal::gpio::DynPinId;
// USB Communications Class Device support
use usbd_serial::SerialPort;

const MAX_PINS: usize = 30;

type PinO = rp2040_hal::gpio::Pin<
    rp2040_hal::gpio::DynPinId,
    rp2040_hal::gpio::FunctionSio<rp2040_hal::gpio::SioOutput>,
    rp2040_hal::gpio::DynPullType,
>;
const PINO_NONE: Option<PinO> = None;
type PinI = rp2040_hal::gpio::Pin<
    rp2040_hal::gpio::DynPinId,
    rp2040_hal::gpio::FunctionSio<rp2040_hal::gpio::SioInput>,
    rp2040_hal::gpio::DynPullType,
>;
const PINI_NONE: Option<PinI> = None;

/// Application Digital I/O
pub struct DioRequestProcessor {
    pins_id: [Option<DynPinId>; MAX_PINS],
    pins_o: [Option<PinO>; MAX_PINS],
    pins_i: [Option<PinI>; MAX_PINS],
}

impl DioRequestProcessor {
    /// Create a new instance of the DioRequestProcessor
    ///
    pub fn new(pins_id: [Option<DynPinId>; MAX_PINS]) -> Self {
        DioRequestProcessor {
            pins_id: pins_id,
            pins_o: [PINO_NONE; MAX_PINS],
            pins_i: [PINI_NONE; MAX_PINS],
        }
    }

    /// Set a pin as output
    ///
    fn set_pin_as_output(&mut self, pin_num: u32) {
        print_debug_message!("\tset pin {:?} as output", pin_num);
        self.pins_id[pin_num as usize]
            .map(|dyn_id| unsafe {
                let pin = new_pin(dyn_id);
                pin.try_into_function::<rp2040_hal::gpio::FunctionSioOutput>()
                    .and_then(|pin_out| {
                        self.pins_o[pin_num as usize] = Some(pin_out);
                        Ok(())
                    })
                    // Ignore the error, just a warning
                    .map_err(|_| {
                        print_debug_message!(
                            "      * error converting pin {:?} to output",
                            pin_num
                        );
                    })
                    .ok();
            })
            // Ignore the error, just a warning
            .ok_or_else(|| {
                print_debug_message!("      * pin {:?} not available", pin_num);
            })
            .ok();
    }

    /// Set a pin as input
    ///
    fn set_pin_as_input(&mut self, pin_num: u32) {
        print_debug_message!("\tset pin {:?} as input", pin_num);
        self.pins_id[pin_num as usize]
            .map(|dyn_id| unsafe {
                let pin = new_pin(dyn_id);
                pin.try_into_function::<rp2040_hal::gpio::FunctionSioInput>()
                    .and_then(|mut pin_in| {
                        // pin_in.set_pull_type(rp2040_hal::gpio::DynPullType::None);
                        pin_in.set_pull_type(rp2040_hal::gpio::DynPullType::Down);
                        self.pins_i[pin_num as usize] = Some(pin_in);
                        Ok(())
                    })
                    // Ignore the error, just a warning
                    .map_err(|_| {
                        print_debug_message!("      * error converting pin {:?} to input", pin_num);
                    })
                    .ok();
            })
            // Ignore the error, just a warning
            .ok_or_else(|| {
                print_debug_message!("      * pin {:?} not available", pin_num);
            })
            .ok();
    }

    /// Set a pin low
    ///
    fn set_pin_low(&mut self, pin_num: u32) -> Result<(), &'static str> {
        self.pins_o[pin_num as usize]
            .as_mut()
            .map(|pin| {
                pin.set_low().unwrap();
            })
            .ok_or_else(|| {
                print_debug_message!("\t!!!pin {:?} not available", pin_num);
                "Pin not available"
            })?;
        Ok(())
    }

    /// Set a pin high
    ///
    fn set_pin_high(&mut self, pin_num: u32) -> Result<(), &'static str> {
        self.pins_o[pin_num as usize]
            .as_mut()
            .map(|pin| {
                pin.set_high().unwrap();
            })
            .ok_or_else(|| {
                print_debug_message!("\t!!!pin {:?} not available", pin_num);
                "Pin not available"
            })?;
        Ok(())
    }

    ///
    ///
    pub fn process_request(
        &mut self,
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: PicohaDioRequest,
    ) {
        print_debug_message!("+ processing request: {:?}", request);

        match request.r#type {
            femtopb::EnumValue::Known(k) => match k {
                crate::api_dio::RequestType::Ping => Self::process_request_ping(serial),
                crate::api_dio::RequestType::SetPinDirection => {
                    self.process_request_set_pin_direction(serial, request)
                }
                crate::api_dio::RequestType::SetPinValue => {
                    self.process_request_set_pin_value(serial, request)
                }
                crate::api_dio::RequestType::GetPinDirection => {
                    self.process_request_get_pin_direction(serial, request)
                }
                crate::api_dio::RequestType::GetPinValue => {
                    self.process_request_get_pin_value(serial, request)
                }
            },
            femtopb::EnumValue::Unknown(_) => todo!(),
        }
    }

    /// Process a ping request
    ///
    fn process_request_ping(serial: &mut SerialPort<rp2040_hal::usb::UsbBus>) {
        print_debug_message!(b"\t* processing request: PING\r\n");
        let mut answer = PicohaDioAnswer::default();
        answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);
        Self::send_answer(serial, answer);
    }

    /// Process a set pin direction request
    ///
    fn process_request_set_pin_direction(
        &mut self,
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: PicohaDioRequest,
    ) {
        print_debug_message!(b"      * processing request: SET_PIN_DIRECTION\r\n");

        match request.value {
            femtopb::EnumValue::Known(v) => match v {
                crate::api_dio::PinValue::Input => self.set_pin_as_input(request.pin_num),
                crate::api_dio::PinValue::Output => self.set_pin_as_output(request.pin_num),
                _ => {
                    print_debug_message!("      * invalid value: {:?}", v);
                }
            },
            femtopb::EnumValue::Unknown(_) => todo!(),
        }

        let mut answer = PicohaDioAnswer::default();
        answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);
        Self::send_answer(serial, answer);
    }

    /// Process a set pin value request
    ///
    fn process_request_set_pin_value(
        &mut self,
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: PicohaDioRequest,
    ) {
        print_debug_message!(b"\tprocessing request: SET_PIN_VALUE\r\n");
        let r = match request.value {
            femtopb::EnumValue::Known(v) => match v {
                crate::api_dio::PinValue::Low => self.set_pin_low(request.pin_num),
                crate::api_dio::PinValue::High => self.set_pin_high(request.pin_num),
                _ => {
                    print_debug_message!("\t!!! invalid value: {:?}", v);
                    Err("Invalid value")
                }
            },
            femtopb::EnumValue::Unknown(_) => todo!(),
        };

        let mut answer = PicohaDioAnswer::default();
        match r {
            Ok(_) => {
                answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);
            }
            Err(e) => {
                answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Failure);
                answer.error_message = Some(e);
            }
        }
        Self::send_answer(serial, answer);
    }

    fn process_request_get_pin_direction(
        &mut self,
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: PicohaDioRequest,
    ) {
        print_debug_message!(b"      * processing request: GET_PIN_DIRECTION\r\n");
        let mut answer = PicohaDioAnswer::default();
        answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);
        Self::send_answer(serial, answer);
    }

    fn process_request_get_pin_value(
        &mut self,
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: PicohaDioRequest,
    ) {
        print_debug_message!(b"      * processing request: GET_PIN_VALUE\r\n");
        let mut answer = PicohaDioAnswer::default();
        answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);
        Self::send_answer(serial, answer);
    }

    /// Send an answer
    ///
    fn send_answer(serial: &mut SerialPort<rp2040_hal::usb::UsbBus>, answer: PicohaDioAnswer) {
        let mut buffer = [0u8; 64];
        let encoded_len = answer.encoded_len();
        answer.encode(&mut buffer.as_mut()).unwrap();

        // print_debug_message!("      * sending answer: {:?}", encoded_len);
        // print_debug_message!("      * sending answer: {:?}", &buffer[..encoded_len]);

        // Prepare encoding
        let mut encoded_command = [0u8; 1024];
        let mut slip_encoder = serial_line_ip::Encoder::new();

        // Encode the command
        let mut totals = match slip_encoder.encode(&buffer[..encoded_len], &mut encoded_command) {
            Ok(t) => t,
            Err(e) => {
                print_debug_message!("      * error encoding answer: {:?}", e);
                return;
            }
        };

        // Finalise the encoding
        totals += match slip_encoder.finish(&mut encoded_command[totals.written..]) {
            Ok(t) => t,
            Err(e) => {
                print_debug_message!("      * error encoding answer: {:?}", e);
                return;
            }
        };

        // print_debug_message!("      * sending answer: {:?}", totals.written);
        print_debug_message!(
            "      * sending answer: {:?}",
            &encoded_command[..totals.written]
        );

        match serial.write(&encoded_command[..totals.written]) {
            Ok(_) => print_debug_message!(b"      * answer sent\r\n"),
            Err(e) => print_debug_message!("      * answer not sent {:?}", e),
        }
    }
}
