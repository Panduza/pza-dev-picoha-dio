// Print debug support
use crate::api_dio_utils;
use crate::uart_debug::uart_debug_print;
use crate::{api_dio::PicohaDioAnswer, api_dio::PicohaDioRequest, print_debug_message};
use core::fmt::Write;

// Message deserialization support
use femtopb::Message;

// USB Communications Class Device support
use usbd_serial::SerialPort;

// Size of internal buffers
const BUFFER_CAPACITY: usize = 64;

/// Application Digital I/O
pub struct AppDio {
    // Accumulated incoming data buffer
    in_buf: [u8; BUFFER_CAPACITY],
    // Keep track of number of data in the buffer
    in_buf_size: usize,
    // Decode buffer
    decode_buffer: [u8; BUFFER_CAPACITY],
}

impl AppDio {
    /// Create a new instance of the AppDio
    ///
    pub fn new() -> Self {
        AppDio {
            in_buf: [0u8; 64],
            in_buf_size: 0,
            decode_buffer: [0u8; 64],
        }
    }

    /// Accumulate new data
    ///
    fn accumulate_new_data(&mut self, data: &[u8]) {
        // Compute indexes
        let data_len = data.len();
        let i_from = self.in_buf_size;
        let i_to = self.in_buf_size + data_len;

        // Copy data to the buffer
        self.in_buf[i_from..i_to].clone_from_slice(&data);

        // Update the buffer size
        self.in_buf_size += data_len;
    }

    /// Try to decode an API request
    ///
    fn try_to_decode_api_request(frame: &[u8]) -> Option<PicohaDioRequest> {
        match PicohaDioRequest::decode(frame) {
            Ok(ppp) => {
                let mut new_request = PicohaDioRequest::default();
                new_request.r#type = ppp.r#type;
                new_request.pin_num = ppp.pin_num;
                new_request.value = ppp.value;
                Some(new_request)
            }
            Err(e) => None,
        }
    }

    /// Try to decode a request from the incoming data buffer
    ///
    fn try_to_decode_buffer(&mut self) -> Option<PicohaDioRequest> {
        let mut slip_decoder = serial_line_ip::Decoder::new();

        match slip_decoder.decode(&self.in_buf[..self.in_buf_size], &mut self.decode_buffer) {
            Ok((input_bytes_processed, output_slice, is_end_of_packet)) => {
                if is_end_of_packet {
                    Self::try_to_decode_api_request(output_slice)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    ///
    ///
    fn process_request(
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: PicohaDioRequest,
    ) {
        print_debug_message!("+ processing request: {:?}", request);

        match request.r#type {
            femtopb::EnumValue::Known(k) => match k {
                crate::api_dio::RequestType::Ping => Self::process_request_ping(serial),
                crate::api_dio::RequestType::SetPinDirection => {
                    Self::process_request_set_pin_direction(serial, request)
                }
                crate::api_dio::RequestType::SetPinValue => {
                    Self::process_request_set_pin_value(serial, request)
                }
                crate::api_dio::RequestType::GetPinDirection => {
                    Self::process_request_get_pin_direction(serial, request)
                }
                crate::api_dio::RequestType::GetPinValue => {
                    Self::process_request_get_pin_value(serial, request)
                }
            },
            femtopb::EnumValue::Unknown(_) => todo!(),
        }
    }

    /// Process a ping request
    ///
    fn process_request_ping(serial: &mut SerialPort<rp2040_hal::usb::UsbBus>) {
        print_debug_message!(b"      * processing request: PING");
        let mut answer = PicohaDioAnswer::default();
        answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);
        Self::send_answer(serial, answer);
    }

    fn process_request_set_pin_direction(
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: PicohaDioRequest,
    ) {
        print_debug_message!(b"      * processing request: SET_PIN_DIRECTION");
        let mut answer = PicohaDioAnswer::default();
        answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);
        Self::send_answer(serial, answer);
    }

    fn process_request_set_pin_value(
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: PicohaDioRequest,
    ) {
        print_debug_message!(b"      * processing request: SET_PIN_VALUE");
        let mut answer = PicohaDioAnswer::default();
        answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);
        Self::send_answer(serial, answer);
    }

    fn process_request_get_pin_direction(
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: PicohaDioRequest,
    ) {
        print_debug_message!(b"      * processing request: GET_PIN_DIRECTION");
        let mut answer = PicohaDioAnswer::default();
        answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Success);
        Self::send_answer(serial, answer);
    }

    fn process_request_get_pin_value(
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        request: PicohaDioRequest,
    ) {
        print_debug_message!(b"      * processing request: GET_PIN_VALUE");
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
        serial.write(&buffer[..encoded_len]).unwrap();
    }

    /// Process incoming data
    ///
    pub fn process_incoming_data(
        &mut self,
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        data: &[u8],
    ) {
        print_debug_message!("+ recieved data: {:?}", data);
        self.accumulate_new_data(data);
        while self.try_to_decode_buffer().is_some() {
            if let Some(request) = self.try_to_decode_buffer() {
                print_debug_message!("+ decoded request: {:?}", request);
                Self::process_request(serial, request);
            }
        }
    }
}
