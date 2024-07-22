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
pub struct AppDio {
    // Accumulated incoming data buffer
    in_buf: [u8; BUFFER_CAPACITY],
    // Keep track of number of data in the buffer
    in_buf_size: usize,
    // Decode buffer
    decode_buffer: [u8; BUFFER_CAPACITY],

    // rp_pins: rp_pico::Pins,
    pins_o: [Option<PinO>; MAX_PINS],
    pins_i: [Option<PinI>; MAX_PINS],
}

impl AppDio {
    /// Create a new instance of the AppDio
    ///
    pub fn new() -> Self {
        AppDio {
            in_buf: [0u8; 64],
            in_buf_size: 0,
            decode_buffer: [0u8; 64],
            // rp_pins: rp_pins,
            pins_o: [PINO_NONE; MAX_PINS],
            pins_i: [PINI_NONE; MAX_PINS],
        }
    }

    // fn set_pin_as_output(&mut self, pin_num: u32) {
    //     let pin_num = pin_num as usize;
    //     if pin_num < MAX_PINS {
    //         self.pins_o[pin_num] = Some(pin.into_push_pull_output().into_dyn_pin());
    //     }
    // }

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
            Err(e) => {
                print_debug_message!("      * error decoding request: {:?}", e);
                None
            }
        }
    }

    /// Try to decode a request from the incoming data buffer
    ///
    fn try_to_decode_buffer(&mut self) -> Option<PicohaDioRequest> {
        let mut slip_decoder = serial_line_ip::Decoder::new();

        // Try to decode
        let mut in_size: usize = 0;
        let mut out_slice = [0u8; 64];
        let mut out_size = 0;
        let mut is_end_of_packet: bool = false;

        {
            match slip_decoder.decode(
                self.in_buf[..self.in_buf_size].as_mut(),
                &mut self.decode_buffer,
            ) {
                Ok((input_bytes_processed, out, is_eop)) => {
                    in_size = input_bytes_processed;
                    out_size = out.len();
                    out_slice[..out_size].clone_from_slice(out);
                    is_end_of_packet = is_eop;
                }
                Err(e) => {
                    print_debug_message!("      * error decoding request: {:?}", e);
                    return None;
                }
            }
        }

        // Debug
        print_debug_message!("      * {:?} {:?}", in_size, is_end_of_packet);
        print_debug_message!("      * - {:?} ", self.decode_buffer[..out_size].as_ref());

        // Check if we have a complete packet
        if is_end_of_packet {
            // Shift data inside the in_buf to the left
            let in_buf = &mut self.in_buf;
            in_buf.copy_within(in_size..self.in_buf_size, 0);
            self.in_buf_size -= in_size;

            let pppp = self.decode_buffer[..out_size].as_ref();
            let val = Self::try_to_decode_api_request(&pppp);
            return val;
        } else {
            None
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
        answer.r#type = femtopb::EnumValue::Known(crate::api_dio::AnswerType::Failure);
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

        print_debug_message!("      * sending answer: {:?}", encoded_len);
        print_debug_message!("      * sending answer: {:?}", &buffer[..encoded_len]);

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

        print_debug_message!("      * sending answer 2: {:?}", totals.written);
        print_debug_message!(
            "      * sending answer 2: {:?}",
            &encoded_command[..totals.written]
        );

        match serial.write(&encoded_command[..totals.written]) {
            Ok(_) => print_debug_message!(b"      * answer sent"),
            Err(e) => print_debug_message!("      * answer not sent {:?}", e),
        }
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
        while let Some(request) = self.try_to_decode_buffer() {
            print_debug_message!("+ decoded request: {:?}", request);
            Self::process_request(serial, request);
        }
    }
}
