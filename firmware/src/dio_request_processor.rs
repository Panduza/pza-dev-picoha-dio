use crate::api_dio::{PicohaDioAnswer, PicohaDioRequest};
use crate::debug;
use embassy_rp::usb::{Driver, Instance};
use embassy_usb::class::cdc_acm::CdcAcmClass;

// Message deserialization support
use femtopb::Message;

use embassy_rp::gpio::{Flex, Level, Pull};

use crate::MAX_PINS;

#[derive(PartialEq)]
enum PinDirection {
    Input = 0,
    Output = 1,
}

/// Application Digital I/O
pub struct DioRequestProcessor<'a, 'b> {
    pins: &'a mut [Option<Flex<'b>>; MAX_PINS],
    directions: [PinDirection; MAX_PINS],
}

const DEFAULT_DIRECTION: PinDirection = PinDirection::Input;

impl<'a, 'b> DioRequestProcessor<'a, 'b> {
    /// Create a new instance of the DioRequestProcessor
    ///
    pub fn new(pins: &'a mut [Option<Flex<'b>>; MAX_PINS]) -> Self {
        let mut processor = DioRequestProcessor {
            pins: pins,
            directions: [DEFAULT_DIRECTION; MAX_PINS],
        };

        let _ = processor.set_all_pin_as_input();
        processor
    }

    /// Set all pins as input
    ///
    fn set_all_pin_as_input(&mut self) -> Result<(), &'static str> {
        for pin_value in self.pins.iter_mut() {
            if let Some(pin) = pin_value {
                pin.set_as_input();
                pin.set_pull(Pull::Down);
            }
        }

        Ok(())
    }

    /// Set a pin as output
    ///
    fn set_pin_as_output(&mut self, pin_num: u32) -> Result<(), &'static str> {
        debug!("\tset pin {:?} as output", pin_num);
        if let Some(pin) = &mut self.pins[pin_num as usize] {
            pin.set_as_output();
            pin.set_low();
            self.directions[pin_num as usize] = PinDirection::Output;
        }

        Ok(())
    }

    /// Set a pin as input
    ///
    fn set_pin_as_input(&mut self, pin_num: u32) -> Result<(), &'static str> {
        debug!("\tset pin {:?} as input", pin_num);

        if let Some(pin) = &mut self.pins[pin_num as usize] {
            pin.set_as_input();
            pin.set_pull(Pull::Down);
            self.directions[pin_num as usize] = PinDirection::Input;
        }

        Ok(())
    }

    /// Set a pin low
    ///
    fn set_pin_low(&mut self, pin_num: u32) -> Result<(), &'static str> {
        if self.directions[pin_num as usize] != PinDirection::Output {
            debug!("\tError: pin {:?} is input", pin_num);
            return Err("Pin is input");
        }

        debug!("\t+pin {:?} low", pin_num);
        if let Some(pin) = &mut self.pins[pin_num as usize] {
            pin.set_low();
        }

        Ok(())
    }

    /// Set a pin high
    ///
    fn set_pin_high(&mut self, pin_num: u32) -> Result<(), &'static str> {
        if self.directions[pin_num as usize] != PinDirection::Output {
            debug!("\tError: pin {:?} is input", pin_num);

            return Err("Pin is input");
        }

        debug!("\t+pin {:?} low", pin_num);
        if let Some(pin) = &mut self.pins[pin_num as usize] {
            pin.set_high();
        }

        Ok(())
    }

    /// Process a request, main entry point
    ///
    pub async fn process_request<'d, T: Instance + 'd>(
        &mut self,
        serial: &mut CdcAcmClass<'d, Driver<'d, T>>,
        request: &PicohaDioRequest<'_>,
    ) {
        debug!("+ processing request: {:?}", request);

        // Default response
        let mut answer = PicohaDioAnswer::default();
        answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);

        // Check pin index
        if let femtopb::EnumValue::Known(req_type) = request.r#type {
            if req_type != crate::api_dio::RequestType::Ping {
                if request.pin_num >= self.pins.len() as u32
                    || self.pins[request.pin_num as usize].is_none()
                {
                    debug!("\tInvalid pin {:?}", request.pin_num);

                    answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Failure);
                    answer.error_message = Some("Invalid pin");
                    let _ = self.send_answer(serial, &mut answer).await;
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

        let _ = self.send_answer(serial, &mut answer).await;
    }

    /// Process a ping request
    ///
    fn process_request_ping(&mut self, _answer: &mut PicohaDioAnswer) -> Result<(), &'static str> {
        debug!("\t* processing request: PING");
        Ok(())
    }

    /// Process a set pin direction request
    ///
    fn process_request_set_pin_direction(
        &mut self,
        request: &PicohaDioRequest,
        _answer: &mut PicohaDioAnswer,
    ) -> Result<(), &'static str> {
        debug!("      * processing request: SET_PIN_DIRECTION");

        match request.value {
            femtopb::EnumValue::Known(v) => match v {
                crate::api_dio::PinValue::Input => self.set_pin_as_input(request.pin_num),
                crate::api_dio::PinValue::Output => self.set_pin_as_output(request.pin_num),
                _ => {
                    debug!("      * invalid value: {:?}", v);
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
        debug!("\tprocessing request: SET_PIN_VALUE");

        //
        // Process the request
        match request.value {
            femtopb::EnumValue::Known(v) => match v {
                crate::api_dio::PinValue::Low => self.set_pin_low(request.pin_num),
                crate::api_dio::PinValue::High => self.set_pin_high(request.pin_num),
                _ => {
                    debug!("      * invalid value: {:?}", v);
                    Err("Invalid value")
                }
            },
            femtopb::EnumValue::Unknown(_) => Err("Invalid value"),
        }
    }

    /// Process an incoming request to get a pin direction
    ///
    fn process_request_get_pin_direction(
        &mut self,
        request: &PicohaDioRequest,
        answer: &mut PicohaDioAnswer,
    ) -> Result<(), &'static str> {
        debug!("      * processing request: GET_PIN_DIRECTION");

        match self.directions[request.pin_num as usize] {
            PinDirection::Input => {
                debug!("      * input");
                answer.value = Some(femtopb::EnumValue::Known(crate::api_dio::PinValue::Input));
            }
            PinDirection::Output => {
                debug!("      * output");
                answer.value = Some(femtopb::EnumValue::Known(crate::api_dio::PinValue::Output));
            }
        }

        Ok(())
    }

    fn process_request_get_pin_value(
        &mut self,
        request: &PicohaDioRequest,
        answer: &mut PicohaDioAnswer,
    ) -> Result<(), &'static str> {
        debug!("      * processing request: GET_PIN_VALUE");

        if let Some(pin) = &self.pins[request.pin_num as usize] {
            let level = {
                match self.directions[request.pin_num as usize] {
                    PinDirection::Input => pin.get_level(),
                    PinDirection::Output => pin.get_output_level(),
                }
            };

            match level {
                Level::High => {
                    debug!("      * high");
                    answer.value = Some(femtopb::EnumValue::Known(crate::api_dio::PinValue::High));
                }
                Level::Low => {
                    debug!("      * low");
                    answer.value = Some(femtopb::EnumValue::Known(crate::api_dio::PinValue::Low));
                }
            }
        }

        Ok(())
    }

    /// Send an answer
    ///
    async fn send_answer<'d, T: Instance + 'd>(
        &mut self,
        serial: &mut CdcAcmClass<'d, Driver<'d, T>>,
        answer: &PicohaDioAnswer<'_>,
    ) -> Result<(), u32> {
        let mut buffer = [0u8; 64];
        let encoded_len = answer.encoded_len();
        answer.encode(&mut buffer.as_mut()).or_else(|e| {
            debug!("      * error encoding answer: {:?}", e);
            Err(1 as u32)
        })?;

        debug!("      * answer: {:?}", buffer);
        // debug!("      * sending answer: {:?}", encoded_len);
        // debug!("      * sending answer: {:?}", &buffer[..encoded_len]);

        // Prepare encoding
        let mut encoded_command = [0u8; 1024];
        let mut slip_encoder = serial_line_ip::Encoder::new();

        // Encode the command
        let mut totals = slip_encoder
            .encode(&buffer[..encoded_len], &mut encoded_command)
            .or_else(|e| {
                debug!("      * error encoding answer: {:?}", e);
                Err(2 as u32)
            })?;

        // Finalise the encoding
        totals += slip_encoder
            .finish(&mut encoded_command[totals.written..])
            .or_else(|e| {
                debug!("      * error encoding answer: {:?}", e);
                Err(3 as u32)
            })?;

        // debug!("      * sending answer: {:?}", totals.written);
        debug!(
            "      * sending answer: {:?}",
            &encoded_command[..totals.written]
        );

        let res = serial
            .write_packet(&encoded_command[..totals.written])
            .await;

        match res {
            Ok(_) => {
                debug!("      * answer sent");
                Ok(())
            }
            Err(e) => {
                debug!("      * answer not sent {:?}", e);
                Err(4 as u32)
            }
        }
    }
}
