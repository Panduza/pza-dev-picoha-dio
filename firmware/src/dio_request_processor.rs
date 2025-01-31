// Print debug support
use crate::uart_debug::UartType;
use crate::{
    api_dio::{PicohaDioAnswer, PicohaDioRequest},
    print_debug_message,
};
#[cfg(any(feature = "uart0_debug"))]
use core::fmt::Write;

use embedded_hal::digital::{InputPin, OutputPin};
use rp2040_hal::gpio::new_pin;
// Message deserialization support
use femtopb::Message;

use rp2040_hal::gpio::DynPinId;
// USB Communications Class Device support
use usbd_serial::SerialPort;

const MAX_PINS: usize = 23;

#[derive(PartialEq)]
enum PinDirection {
    Input = 0,
    Output = 1,
}

#[derive(PartialEq)]
enum PinValue {
    Low = 0,
    High = 1,
}

struct Pin {
    id: Option<DynPinId>,
    direction: PinDirection,
    value: PinValue,
}

const DEFAULT_PIN: Pin = Pin {
    id: None,
    direction: PinDirection::Input,
    value: PinValue::Low,
};

/// Application Digital I/O
pub struct DioRequestProcessor<'a> {
    debug_uart: &'a Option<UartType>,
    pins: [Pin; MAX_PINS],
}

impl<'a> DioRequestProcessor<'a> {
    /// Create a new instance of the DioRequestProcessor
    ///
    pub fn new(
        debug_uart: &'a Option<UartType>,
        pins_id: &'a [Option<DynPinId>; MAX_PINS],
    ) -> Self {
        let mut processor = DioRequestProcessor {
            debug_uart: debug_uart,
            pins: [DEFAULT_PIN; MAX_PINS],
        };

        for (idx, pin_id) in pins_id.iter().enumerate() {
            processor.pins[idx].id = *pin_id;
        }

        processor
    }

    ///
    /// Initialize all pins as input
    ///
    pub fn init_all_pins_as_input(&mut self) {
        for idx in 0..self.pins.len() {
            let _ = self.set_pin_as_input(idx as u32);
        }
    }

    /// Set a pin as output
    ///
    fn set_pin_as_output(&mut self, pin_num: u32) -> Result<(), &'static str> {
        print_debug_message!(self.debug_uart, "\tset pin {:?} as output", pin_num);
        if let Some(dyn_id) = self.pins[pin_num as usize].id {
            unsafe {
                let pin = new_pin(dyn_id);
                pin.try_into_function::<rp2040_hal::gpio::FunctionSioOutput>()
                    .and_then(|mut pin_out| {
                        let _ = pin_out.set_low();
                        self.pins[pin_num as usize].direction = PinDirection::Output;
                        Ok(())
                    })
                    .or_else(|_| {
                        print_debug_message!(
                            self.debug_uart,
                            "      * error converting pin {:?} to output",
                            pin_num
                        );
                        Err("Set output failed")
                    })?;
            }
        };

        Ok(())
    }

    /// Set a pin as input
    ///
    fn set_pin_as_input(&mut self, pin_num: u32) -> Result<(), &'static str> {
        print_debug_message!(self.debug_uart, "\tset pin {:?} as input", pin_num);

        if let Some(dyn_id) = self.pins[pin_num as usize].id {
            unsafe {
                let pin = new_pin(dyn_id);
                pin.try_into_function::<rp2040_hal::gpio::FunctionSioInput>()
                    .and_then(|mut pin_in| {
                        pin_in.set_pull_type(rp2040_hal::gpio::DynPullType::Down);
                        self.pins[pin_num as usize].direction = PinDirection::Input;
                        Ok(())
                    })
                    .or_else(|_| {
                        print_debug_message!(
                            self.debug_uart,
                            "      * error converting pin {:?} to input",
                            pin_num
                        );
                        Err("Set input failed")
                    })?;
            }
        }

        Ok(())
    }

    /// Set a pin low
    ///
    fn set_pin_low(&mut self, pin_num: u32) -> Result<(), &'static str> {
        if self.pins[pin_num as usize].direction != PinDirection::Output {
            print_debug_message!(self.debug_uart, "\tError: pin {:?} is input", pin_num);
            return Err("Pin is input");
        }

        print_debug_message!(self.debug_uart, "\t+pin {:?} low", pin_num);
        if let Some(dyn_id) = self.pins[pin_num as usize].id {
            unsafe {
                let pin = new_pin(dyn_id);
                pin.try_into_function::<rp2040_hal::gpio::FunctionSioOutput>()
                    .and_then(|mut pin_out| {
                        let _ = pin_out.set_low();
                        self.pins[pin_num as usize].value = PinValue::Low;
                        Ok(())
                    })
                    .or_else(|_| {
                        print_debug_message!(
                            self.debug_uart,
                            "\t!!!pin {:?} not available",
                            pin_num
                        );
                        Err("Unable to set pin as low")
                    })?;
            }
        }

        Ok(())
    }

    /// Set a pin high
    ///
    fn set_pin_high(&mut self, pin_num: u32) -> Result<(), &'static str> {
        if self.pins[pin_num as usize].direction != PinDirection::Output {
            print_debug_message!(self.debug_uart, "\tError: pin {:?} is input", pin_num);

            return Err("Pin is input");
        }

        print_debug_message!(self.debug_uart, "\t+pin {:?} low", pin_num);
        if let Some(dyn_id) = self.pins[pin_num as usize].id {
            unsafe {
                let pin = new_pin(dyn_id);
                pin.try_into_function::<rp2040_hal::gpio::FunctionSioOutput>()
                    .and_then(|mut pin_out| {
                        let _ = pin_out.set_high();
                        self.pins[pin_num as usize].value = PinValue::High;
                        Ok(())
                    })
                    .or_else(|_| {
                        print_debug_message!(
                            self.debug_uart,
                            "\t!!!pin {:?} not available",
                            pin_num
                        );
                        Err("Unable to set pin as high")
                    })?;
            }
        }

        Ok(())
    }

    /// Process a request, main entry point
    ///
    pub fn process_request(
        &mut self,
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: &PicohaDioRequest,
    ) {
        //
        // Debug log
        print_debug_message!(self.debug_uart, "+ processing request: {:?}", request);

        // Default response
        let mut answer = PicohaDioAnswer::default();
        answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);

        // Check pin index
        if let femtopb::EnumValue::Known(req_type) = request.r#type {
            if req_type != crate::api_dio::RequestType::Ping {
                if request.pin_num >= self.pins.len() as u32
                    || self.pins[request.pin_num as usize].id.is_none()
                {
                    print_debug_message!(self.debug_uart, "\tInvalid pin {:?}", request.pin_num);

                    answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Failure);
                    answer.error_message = Some("Invalid pin");
                    let _ = self.send_answer(serial, &mut answer);
                    return;
                }
            }
        }

        //
        // Choose the correct process function
        let r = match request.r#type {
            femtopb::EnumValue::Known(k) => match k {
                crate::api_dio::RequestType::Ping => self.process_request_ping(&mut answer),
                crate::api_dio::RequestType::SetPinDirection => {
                    self.process_request_set_pin_direction(request, &mut answer)
                }
                crate::api_dio::RequestType::SetPinValue => {
                    self.process_request_set_pin_value(request, &mut answer)
                }
                crate::api_dio::RequestType::GetPinDirection => {
                    self.process_request_get_pin_direction(request, &mut answer)
                }
                crate::api_dio::RequestType::GetPinValue => {
                    self.process_request_get_pin_value(request, &mut answer)
                }
            },
            femtopb::EnumValue::Unknown(_) => Err("Invalid value"),
        };

        let _ = r.or_else(|e| {
            answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Failure);
            answer.error_message = Some(e);
            Err(e)
        });

        let _ = self.send_answer(serial, &mut answer);
    }

    /// Process a ping request
    ///
    fn process_request_ping(&self, _answer: &mut PicohaDioAnswer) -> Result<(), &'static str> {
        print_debug_message!(self.debug_uart, b"\t* processing request: PING\r\n");
        Ok(())
    }

    /// Process a set pin direction request
    ///
    fn process_request_set_pin_direction(
        &mut self,
        request: &PicohaDioRequest,
        _answer: &mut PicohaDioAnswer,
    ) -> Result<(), &'static str> {
        print_debug_message!(
            self.debug_uart,
            b"      * processing request: SET_PIN_DIRECTION\r\n"
        );

        match request.value {
            femtopb::EnumValue::Known(v) => match v {
                crate::api_dio::PinValue::Input => self.set_pin_as_input(request.pin_num),
                crate::api_dio::PinValue::Output => self.set_pin_as_output(request.pin_num),
                _ => {
                    print_debug_message!(self.debug_uart, "      * invalid value: {:?}", v);
                    Err("Invalid value")
                }
            },
            femtopb::EnumValue::Unknown(_) => Err("Invalid value"),
        }
    }

    /// Process a set pin value request
    ///
    fn process_request_set_pin_value(
        &mut self,
        request: &PicohaDioRequest,
        _answer: &mut PicohaDioAnswer,
    ) -> Result<(), &'static str> {
        //
        // Debug log
        print_debug_message!(self.debug_uart, b"\tprocessing request: SET_PIN_VALUE\r\n");

        //
        // Process the request
        match request.value {
            femtopb::EnumValue::Known(v) => match v {
                crate::api_dio::PinValue::Low => self.set_pin_low(request.pin_num),
                crate::api_dio::PinValue::High => self.set_pin_high(request.pin_num),
                _ => {
                    print_debug_message!(self.debug_uart, "      * invalid value: {:?}", v);
                    Err("Invalid value")
                }
            },
            femtopb::EnumValue::Unknown(_) => Err("Invalid value"),
        }
    }

    ///
    /// This function process an incoming request to get a pin direction
    ///
    fn process_request_get_pin_direction(
        &self,
        request: &PicohaDioRequest,
        answer: &mut PicohaDioAnswer,
    ) -> Result<(), &'static str> {
        // Debug log
        print_debug_message!(
            self.debug_uart,
            b"      * processing request: GET_PIN_DIRECTION\r\n"
        );

        match self.pins[request.pin_num as usize].direction {
            PinDirection::Input => {
                print_debug_message!(self.debug_uart, b"      * input\r\n");
                answer.value = Some(femtopb::EnumValue::Known(crate::api_dio::PinValue::Input));
            }
            PinDirection::Output => {
                print_debug_message!(self.debug_uart, b"      * output\r\n");
                answer.value = Some(femtopb::EnumValue::Known(crate::api_dio::PinValue::Output));
            }
        }

        Ok(())
    }

    fn process_request_get_pin_value(
        &self,
        request: &PicohaDioRequest,
        answer: &mut PicohaDioAnswer,
    ) -> Result<(), &'static str> {
        //
        // Debug log
        print_debug_message!(
            self.debug_uart,
            b"      * processing request: GET_PIN_VALUE\r\n"
        );

        match self.pins[request.pin_num as usize].direction {
            PinDirection::Input => {
                if let Some(dyn_id) = self.pins[request.pin_num as usize].id {
                    unsafe {
                        let pin = new_pin(dyn_id);
                        pin.try_into_function::<rp2040_hal::gpio::FunctionSioInput>()
                            .and_then(|mut pin_in| {
                                match pin_in.is_high() {
                                    Ok(true) => {
                                        print_debug_message!(self.debug_uart, b"      * high\r\n");
                                        answer.value = Some(femtopb::EnumValue::Known(
                                            crate::api_dio::PinValue::High,
                                        ));
                                    }
                                    Ok(false) => {
                                        print_debug_message!(self.debug_uart, b"      * low\r\n");
                                        answer.value = Some(femtopb::EnumValue::Known(
                                            crate::api_dio::PinValue::Low,
                                        ));
                                    }
                                    Err(_) => {} // Infaillible
                                }
                                Ok(())
                            })
                            .or_else(|_| {
                                print_debug_message!(
                                    self.debug_uart,
                                    "      * error converting pin {:?} to input",
                                    request.pin_num
                                );

                                Err("Set input failed")
                            })?;
                    }
                }
            }
            PinDirection::Output => match self.pins[request.pin_num as usize].value {
                PinValue::Low => {
                    print_debug_message!(self.debug_uart, b"      * low\r\n");
                    answer.value = Some(femtopb::EnumValue::Known(crate::api_dio::PinValue::Low));
                }
                PinValue::High => {
                    print_debug_message!(self.debug_uart, b"      * high\r\n");
                    answer.value = Some(femtopb::EnumValue::Known(crate::api_dio::PinValue::High));
                }
            },
        };

        Ok(())
    }

    /// Send an answer
    ///
    fn send_answer(
        &self,
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        answer: &PicohaDioAnswer,
    ) -> Result<(), u32> {
        let mut buffer = [0u8; 64];
        let encoded_len = answer.encoded_len();
        answer.encode(&mut buffer.as_mut()).or_else(|e| {
            print_debug_message!(self.debug_uart, "      * error encoding answer: {:?}", e);
            Err(1 as u32)
        })?;

        print_debug_message!(self.debug_uart, "      * answer: {:?}", answer);
        // print_debug_message!(self.debug_uart, "      * sending answer: {:?}", encoded_len);
        // print_debug_message!(self.debug_uart, "      * sending answer: {:?}", &buffer[..encoded_len]);

        // Prepare encoding
        let mut encoded_command = [0u8; 1024];
        let mut slip_encoder = serial_line_ip::Encoder::new();

        // Encode the command
        let mut totals = slip_encoder
            .encode(&buffer[..encoded_len], &mut encoded_command)
            .or_else(|e| {
                print_debug_message!(self.debug_uart, "      * error encoding answer: {:?}", e);
                Err(2 as u32)
            })?;

        // Finalise the encoding
        totals += slip_encoder
            .finish(&mut encoded_command[totals.written..])
            .or_else(|e| {
                print_debug_message!(self.debug_uart, "      * error encoding answer: {:?}", e);
                Err(3 as u32)
            })?;

        // print_debug_message!(self.debug_uart, "      * sending answer: {:?}", totals.written);
        print_debug_message!(
            self.debug_uart,
            "      * sending answer: {:?}",
            &encoded_command[..totals.written]
        );

        let res = serial.write(&encoded_command[..totals.written]);
        match res {
            Ok(_) => {
                print_debug_message!(self.debug_uart, b"      * answer sent\r\n");
                Ok(())
            }
            Err(e) => {
                print_debug_message!(self.debug_uart, "      * answer not sent {:?}", e);
                Err(4 as u32)
            }
        }
    }
}
