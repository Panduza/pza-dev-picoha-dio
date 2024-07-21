// Print debug support
use crate::{api_dio::PicohaDioRequest, print_debug_message};
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
    pub fn new() -> Self {
        AppDio {
            in_buf: [0u8; 64],
            in_buf_size: 0,
            decode_buffer: [0u8; 64],
        }
    }

    /// Accumulate new data
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
    fn try_to_decode_api_request(&mut self, frame: &[u8]) -> Option<PicohaDioRequest> {
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

        slip_decoder
            .decode(&self.in_buf[..self.in_buf_size], &mut self.decode_buffer)
            .map(|(input_bytes_processed, output_slice, is_end_of_packet)| {

                // match  {
                //     Ok(ppp) => {
                //         print_debug_message!("deco {:?}", ppp.pin_num);
                //         Some(ppp)
                //     }
                //     Err(e) => {
                //         print_debug_message!("error deco {:?}", e);
                //         None
                //     }
                // }
            })
            .ok()
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
    }
}
