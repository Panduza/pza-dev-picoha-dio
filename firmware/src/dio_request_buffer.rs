// Print debug support
use crate::api_dio_utils;
use crate::uart_debug::uart_debug_print;
use crate::{api_dio::PicohaDioAnswer, api_dio::PicohaDioRequest, print_debug_message};
use core::fmt::Write;

// Message deserialization support
use femtopb::Message;

// Size of internal buffers
const BUFFER_CAPACITY: usize = 64;

/// Application Digital I/O
pub struct DioRequestBuffer {
    // Accumulated incoming data buffer
    in_buf: [u8; BUFFER_CAPACITY],
    // Keep track of number of data in the buffer
    in_buf_size: usize,
    // Decode buffer
    decode_buffer: [u8; BUFFER_CAPACITY],
}

impl DioRequestBuffer {
    /// Create a new instance of the DioRequestBuffer {
    ///
    pub fn new() -> Self {
        DioRequestBuffer {
            in_buf: [0u8; 64],
            in_buf_size: 0,
            decode_buffer: [0u8; 64],
        }
    }

    /// Accumulate new data
    ///
    pub fn accumulate_new_data(&mut self, data: &[u8]) {
        // Compute indexes
        let data_len = data.len();
        let i_from = self.in_buf_size;
        let i_to = self.in_buf_size + data_len;

        // Copy data to the buffer
        self.in_buf[i_from..i_to].clone_from_slice(&data);

        // Update the buffer size
        self.in_buf_size += data_len;

        //
        let debug_buffer = self.in_buf[..self.in_buf_size].as_ref();
        print_debug_message!("+ accumulated: {:?}", debug_buffer);
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
    pub fn try_to_decode_buffer(&mut self) -> Option<PicohaDioRequest> {
        // Check if we have enough data to decode
        if self.in_buf_size < 2 {
            return None;
        }

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
}
