// Print debug support
use crate::uart_debug::UartType;
use crate::{
    api_dio::{PicohaDioAnswer, PicohaDioRequest},
    print_debug_message,
};
#[cfg(any(feature = "uart0_debug"))]
use core::fmt::Write;

use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use rp2040_hal::gpio::new_pin;
// Message deserialization support
use femtopb::Message;

use rp2040_hal::gpio::DynPinId;
// USB Communications Class Device support
use usbd_serial::SerialPort;

const MAX_PINS: usize = 23;

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

enum PinDirection {
    Input,
    Output,
}
enum PinValue {
    Low,
    High,
}

/// Application Digital I/O
pub struct DioRequestProcessor<'a> {
    debug_uart: &'a Option<UartType>,
    pins_id: &'a [Option<DynPinId>; MAX_PINS],
    pins_o: [Option<PinO>; MAX_PINS],
    pins_i: [Option<PinI>; MAX_PINS],
}

impl<'a> DioRequestProcessor<'a> {
    /// Create a new instance of the DioRequestProcessor
    ///
    pub fn new(
        debug_uart: &'a Option<UartType>,
        pins_id: &'a [Option<DynPinId>; MAX_PINS],
    ) -> Self {
        DioRequestProcessor {
            debug_uart: debug_uart,
            pins_id: pins_id,
            pins_o: [PINO_NONE; MAX_PINS],
            pins_i: [PINI_NONE; MAX_PINS],
        }
    }

    ///
    /// Initialize all pins as input
    ///
    pub fn init_all_pins_as_input(&mut self) {
        for n in 0..MAX_PINS {
            self.set_pin_as_input(n);
        }
    }

    /// Check internal configuration to get the pin direction configuration
    ///
    fn get_internal_pin_direction(&self, pin: usize) -> Option<PinDirection> {
        // Debug
        // print_debug_message!(self.debug_uart, "? check pin {:?}\r\n", pin);

        // if pin is in the output array, it is configured as output
        if self.pins_o[pin].is_some() {
            return Some(PinDirection::Output);
        }

        // if pin is in the input array, it is configured as input
        if self.pins_i[pin].is_some() {
            return Some(PinDirection::Input);
        }

        // else not configured yet
        // print_debug_message!(self.debug_uart, b"? not configured\r\n");
        None
    }

    /// Check internal configuration to get the pin direction configuration
    ///
    fn get_internal_pin_value(&mut self, pin: usize) -> Option<PinValue> {
        // Debug
        print_debug_message!(self.debug_uart, "? check pin {:?}\r\n", pin);

        let dir = self.get_internal_pin_direction(pin);
        match dir {
            Some(d) => match d {
                PinDirection::Input => {
                    let pin_obj = &mut self.pins_i[pin];
                    if let Some(pin_obj) = pin_obj {
                        match pin_obj.is_high() {
                            Ok(true) => return Some(PinValue::High),
                            Ok(false) => return Some(PinValue::Low),
                            Err(_) => {} // Infalible
                        }
                    }
                }
                PinDirection::Output => {
                    print_debug_message!(self.debug_uart, b"      * output ?\r\n");
                    let pin_obj = &mut self.pins_o[pin];
                    if let Some(pin_obj) = pin_obj {
                        match pin_obj.is_set_high() {
                            Ok(true) => return Some(PinValue::High),
                            Ok(false) => return Some(PinValue::Low),
                            Err(_) => {} // Infalible
                        }
                    }
                }
            },
            None => {
                return None;
            }
        }

        // else not configured yet
        // print_debug_message!(self.debug_uart, b"? not configured\r\n");
        None
    }

    /// Set a pin as output
    ///
    fn set_pin_as_output(&mut self, pin_num: usize) {
        print_debug_message!(self.debug_uart, "\tset pin {:?} as output", pin_num);
        self.pins_id[pin_num as usize]
            .map(|dyn_id| unsafe {
                let pin = new_pin(dyn_id);
                pin.try_into_function::<rp2040_hal::gpio::FunctionSioOutput>()
                    .and_then(|pin_out| {
                        //
                        // Remove pin from ouput array if it is there
                        self.pins_i[pin_num] = None;

                        self.pins_o[pin_num as usize] = Some(pin_out);
                        Ok(())
                    })
                    // Ignore the error, just a warning
                    .map_err(|_| {
                        print_debug_message!(
                            self.debug_uart,
                            "      * error converting pin {:?} to output",
                            pin_num
                        );
                    })
                    .ok();
            })
            // Ignore the error, just a warning
            .ok_or_else(|| {
                print_debug_message!(self.debug_uart, "      * pin {:?} not available", pin_num);
            })
            .ok();
    }

    /// Set a pin as input
    ///
    fn set_pin_as_input(&mut self, pin_num: usize) {
        //
        // Debug log
        print_debug_message!(self.debug_uart, "\tset pin {:?} as input", pin_num);

        //
        // Set the pin as input
        self.pins_id[pin_num as usize]
            .map(|dyn_id| unsafe {
                let pin = new_pin(dyn_id);
                pin.try_into_function::<rp2040_hal::gpio::FunctionSioInput>()
                    .and_then(|mut pin_in| {
                        //
                        // Remove pin from ouput array if it is there
                        self.pins_o[pin_num] = None;

                        // pin_in.set_pull_type(rp2040_hal::gpio::DynPullType::None);
                        pin_in.set_pull_type(rp2040_hal::gpio::DynPullType::Down);
                        self.pins_i[pin_num as usize] = Some(pin_in);
                        Ok(())
                    })
                    // Ignore the error, just a warning
                    .map_err(|_| {
                        print_debug_message!(
                            self.debug_uart,
                            "      * error converting pin {:?} to input",
                            pin_num
                        );
                    })
                    .ok();
            })
            // Ignore the error, just a warning
            .ok_or_else(|| {
                print_debug_message!(self.debug_uart, "      * pin {:?} not available", pin_num);
            })
            .ok();
    }

    /// Set a pin low
    ///
    fn set_pin_low(&mut self, pin_num: u32) -> Result<(), &'static str> {
        print_debug_message!(self.debug_uart, "\t+pin {:?} low", pin_num);
        self.pins_o[pin_num as usize]
            .as_mut()
            .map(|pin| {
                pin.set_low().unwrap();
            })
            .ok_or_else(|| {
                print_debug_message!(self.debug_uart, "\t!!!pin {:?} not available", pin_num);
                "Pin not available"
            })?;
        Ok(())
    }

    /// Set a pin high
    ///
    fn set_pin_high(&mut self, pin_num: u32) -> Result<(), &'static str> {
        print_debug_message!(self.debug_uart, "\t+pin {:?} high", pin_num);
        self.pins_o[pin_num as usize]
            .as_mut()
            .map(|pin| {
                pin.set_high().unwrap();
            })
            .ok_or_else(|| {
                print_debug_message!(self.debug_uart, "\t!!!pin {:?} not available", pin_num);
                "Pin not available"
            })?;
        Ok(())
    }

    /// Process a request, main entry point
    ///
    pub fn process_request(
        &mut self,
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: PicohaDioRequest,
    ) {
        //
        // Debug log
        print_debug_message!(self.debug_uart, "+ processing request: {:?}", request);

        //
        // Choose the correct process function
        match request.r#type {
            femtopb::EnumValue::Known(k) => match k {
                crate::api_dio::RequestType::Ping => self.process_request_ping(serial),
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
    fn process_request_ping(&self, serial: &mut SerialPort<rp2040_hal::usb::UsbBus>) {
        print_debug_message!(self.debug_uart, b"\t* processing request: PING\r\n");
        let mut answer = PicohaDioAnswer::default();
        answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);
        self.send_answer(serial, answer);
    }

    /// Process a set pin direction request
    ///
    fn process_request_set_pin_direction(
        &mut self,
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: PicohaDioRequest,
    ) {
        print_debug_message!(
            self.debug_uart,
            b"      * processing request: SET_PIN_DIRECTION\r\n"
        );

        match request.value {
            femtopb::EnumValue::Known(v) => match v {
                crate::api_dio::PinValue::Input => self.set_pin_as_input(request.pin_num as usize),
                crate::api_dio::PinValue::Output => {
                    self.set_pin_as_output(request.pin_num as usize)
                }
                _ => {
                    print_debug_message!(self.debug_uart, "      * invalid value: {:?}", v);
                }
            },
            femtopb::EnumValue::Unknown(_) => todo!(),
        }

        let mut answer = PicohaDioAnswer::default();
        answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);
        self.send_answer(serial, answer);
    }

    /// Process a set pin value request
    ///
    fn process_request_set_pin_value(
        &mut self,
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: PicohaDioRequest,
    ) {
        //
        // Debug log
        print_debug_message!(self.debug_uart, b"\tprocessing request: SET_PIN_VALUE\r\n");

        //
        // Process the request
        let r = match request.value {
            femtopb::EnumValue::Known(v) => match v {
                crate::api_dio::PinValue::Low => self.set_pin_low(request.pin_num),
                crate::api_dio::PinValue::High => self.set_pin_high(request.pin_num),
                _ => {
                    print_debug_message!(self.debug_uart, "\t!!! invalid value: {:?}", v);
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
        self.send_answer(serial, answer);
    }

    ///
    /// This function process an incoming request to get a pin direction
    ///
    fn process_request_get_pin_direction(
        &mut self,
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: PicohaDioRequest,
    ) {
        // Debug log
        print_debug_message!(
            self.debug_uart,
            b"      * processing request: GET_PIN_DIRECTION\r\n"
        );

        // Prepare a default answer
        let mut answer = PicohaDioAnswer::default();

        // Fill the return message
        // Success if the pin has a direction set
        // Failure if the pin is not already configured
        match self.get_internal_pin_direction(request.pin_num as usize) {
            Some(direction) => {
                answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);
                match direction {
                    PinDirection::Input => {
                        print_debug_message!(self.debug_uart, b"      * input\r\n");
                        answer.value =
                            Some(femtopb::EnumValue::Known(crate::api_dio::PinValue::Input));
                    }
                    PinDirection::Output => {
                        print_debug_message!(self.debug_uart, b"      * output\r\n");
                        answer.value =
                            Some(femtopb::EnumValue::Known(crate::api_dio::PinValue::Output));
                    }
                }
            }
            None => {
                answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Failure);
            }
        }

        // Send back the message
        self.send_answer(serial, answer);
    }

    fn process_request_get_pin_value(
        &mut self,
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: PicohaDioRequest,
    ) {
        //
        // Debug log
        print_debug_message!(
            self.debug_uart,
            b"      * processing request: GET_PIN_VALUE\r\n"
        );

        //
        // Prepare a default answer
        let mut answer = PicohaDioAnswer::default();
        answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);

        //
        // Fill the return message
        match self.get_internal_pin_value(request.pin_num as usize) {
            Some(val) => {
                answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);
                match val {
                    PinValue::Low => {
                        print_debug_message!(self.debug_uart, b"      * low\r\n");
                        answer.value =
                            Some(femtopb::EnumValue::Known(crate::api_dio::PinValue::Low));
                    }
                    PinValue::High => {
                        print_debug_message!(self.debug_uart, b"      * high\r\n");
                        answer.value =
                            Some(femtopb::EnumValue::Known(crate::api_dio::PinValue::High));
                    }
                }
            }
            None => {
                answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Failure);
            }
        }

        //
        // Send back the message
        self.send_answer(serial, answer);
    }

    /// Send an answer
    ///
    fn send_answer(
        &self,
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        answer: PicohaDioAnswer,
    ) {
        let mut buffer = [0u8; 64];
        let encoded_len = answer.encoded_len();
        answer.encode(&mut buffer.as_mut()).unwrap();

        print_debug_message!(self.debug_uart, "      * answer: {:?}", answer);
        // print_debug_message!(self.debug_uart, "      * sending answer: {:?}", encoded_len);
        // print_debug_message!(self.debug_uart, "      * sending answer: {:?}", &buffer[..encoded_len]);

        // Prepare encoding
        let mut encoded_command = [0u8; 1024];
        let mut slip_encoder = serial_line_ip::Encoder::new();

        // Encode the command
        let mut totals = match slip_encoder.encode(&buffer[..encoded_len], &mut encoded_command) {
            Ok(t) => t,
            Err(e) => {
                print_debug_message!(self.debug_uart, "      * error encoding answer: {:?}", e);
                return;
            }
        };

        // Finalise the encoding
        totals += match slip_encoder.finish(&mut encoded_command[totals.written..]) {
            Ok(t) => t,
            Err(e) => {
                print_debug_message!(self.debug_uart, "      * error encoding answer: {:?}", e);
                return;
            }
        };

        // print_debug_message!(self.debug_uart, "      * sending answer: {:?}", totals.written);
        print_debug_message!(
            self.debug_uart,
            "      * sending answer: {:?}",
            &encoded_command[..totals.written]
        );

        match serial.write(&encoded_command[..totals.written]) {
            Ok(_) => print_debug_message!(self.debug_uart, b"      * answer sent\r\n"),
            Err(e) => print_debug_message!(self.debug_uart, "      * answer not sent {:?}", e),
        }
    }
}
