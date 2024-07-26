use std::fmt::Debug;

use super::api_dio::PicohaDioAnswer;
use super::connectors::SerialSettings;
use super::connectors::UsbSettings;

// use super::api_dio::PicohaDioRequest;
// use super::api_dio::RequestType;

use cucumber::World;
// use prost::Message;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::time::timeout;
use tokio_serial::SerialStream;
use tracing;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(World)]
pub struct PiochaWorld {
    pub usb_settings: UsbSettings,
    pub serial_settings: SerialSettings,
    pub serial_stream: Option<SerialStream>,
    // Accumulated incoming data buffer
    pub in_buf: [u8; 512],
    // Keep track of number of data in the buffer
    pub in_buf_size: usize,

    decode_buffer: serial_line_ip::DecoderBuffer<512>,

    pub last_answer: Option<PicohaDioAnswer>,
}

impl Debug for PiochaWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PiochaWorld")
            .field("usb_settings", &self.usb_settings)
            .field("serial_settings", &self.serial_settings)
            .field("serial_stream", &self.serial_stream)
            .field("in_buf", &self.in_buf)
            .field("in_buf_size", &self.in_buf_size)
            // .field("decode_buffer", &self.decode_buffer)
            .field("last_answer", &self.last_answer)
            .finish()
    }
}

impl PiochaWorld {
    /// Lock the connector to write a command then wait for the answers
    ///
    pub async fn write_then_read(
        &mut self,
        command: &[u8],
        response: &mut [u8],
    ) -> Result<usize, String> {
        match self.serial_settings.read_timeout {
            // If the timeout is set, use it
            Some(timeout_value) => {
                return Ok(
                    timeout(timeout_value, self.__write_then_read(command, response))
                        .await
                        .map_err(|e| format!("Timeout reading {:?}", e))??,
                );
            }
            // Else good luck !
            None => {
                return Ok(self.__write_then_read(command, response).await?);
            }
        }
    }

    /// This operation is not provided to the public interface
    /// User must use the timeout version for safety on the platform
    ///
    async fn __write_then_read(
        &mut self,
        command: &[u8],
        response: &mut [u8],
    ) -> Result<usize, String> {
        // Prepare encoding
        let mut encoded_command = [0u8; 1024];
        let mut slip_encoder = serial_line_ip::Encoder::new();

        tracing::info!("Sending command: {:?}", command);

        // Encode the command
        let mut totals = slip_encoder
            .encode(command, &mut encoded_command)
            .map_err(|e| format!("Unable to encode command: {:?}", e))?;

        // Finalise the encoding
        totals += slip_encoder
            .finish(&mut encoded_command[totals.written..])
            .map_err(|e| format!("Unable to finsh command encoding: {:?}", e))?;

        // Send the command
        let _ = self
            .serial_stream
            .as_mut()
            .ok_or_else(|| format!("No serial stream"))?
            .write(&encoded_command[..totals.written])
            .await
            .map_err(|e| format!("Unable to write on serial stream: {}", e));

        // Read the response until "end"
        loop {
            let mut chunk_buffer = [0u8; 512];

            // Read a chunck
            let read_size = self
                .serial_stream
                .as_mut()
                .ok_or_else(|| format!("No serial stream"))?
                .read(&mut chunk_buffer)
                .await
                .map_err(|e| format!("Unable to read on serial stream {:?}", e))?;

            let data = &chunk_buffer[..read_size];
            match self.decode_buffer.feed(data) {
                core::prelude::v1::Ok((nb_bytes_processed, found_trame_complete)) => {
                    if found_trame_complete {
                        // let trame = self.decode_buffer.slice();
                        // let request = try_to_decode_api_request(trame).unwrap();
                        // print_debug_message!("+ process request: {:?}", request);
                        // request_processor.process_request(&mut serial, request);
                        break;
                    }
                }
                _ => {}
            }
        }

        let trame_size = self.decode_buffer.slice().len();
        response[..trame_size].copy_from_slice(self.decode_buffer.slice());

        self.decode_buffer.reset();
        Ok(trame_size)
    }
}

impl std::default::Default for PiochaWorld {
    fn default() -> Self {
        let usb_s = UsbSettings::new().set_vendor(0x16c0).set_model(0x05E1);
        let serial_s = SerialSettings::new()
            .set_port_name_from_usb_settings(&usb_s)
            .unwrap()
            .set_baudrate(9600)
            .set_read_timeout(std::time::Duration::from_secs(5));

        PiochaWorld {
            usb_settings: usb_s,
            serial_settings: serial_s,
            serial_stream: None,
            in_buf: [0u8; 512],
            in_buf_size: 0,
            last_answer: None,
            decode_buffer: serial_line_ip::DecoderBuffer::new(),
        }
    }
}
