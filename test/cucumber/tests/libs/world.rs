use super::connectors::UsbSettings;
use super::connectors::SerialSettings;

use super::api_dio::PicohaDioRequest;
use super::api_dio::RequestType;

use cucumber::{given, when, then, World};
use prost::Message;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::time::timeout;
use tokio_serial::SerialStream;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario. 
#[derive(Debug, World)]
pub struct PiochaWorld {
    pub usb_settings: UsbSettings,
    pub serial_settings: SerialSettings,
    pub serial_stream: Option<SerialStream>,
    // Accumulated incoming data buffer
    pub in_buf: [u8; 512],
    // Keep track of number of data in the buffer
    pub in_buf_size: usize,
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

        // Encode the command
        let mut totals = slip_encoder
            .encode(command, &mut encoded_command)
            .map_err(|e| format!("Unable to encode command: {:?}", e))?;

        // Finalise the encoding
        totals += slip_encoder
            .finish(&mut encoded_command[totals.written..])
            .map_err(|e| format!("Unable to finsh command encoding: {:?}", e))?;


        // Send the command
        let write_result = self
            .serial_stream
            .as_mut()
            .ok_or_else(|| format!("No serial stream"))?
            .write(&encoded_command[..totals.written])
            .await
            .map_err(|e| format!("Unable to write on serial stream: {}", e));


        // Read the response until "end"
        loop {
            // Read a chunck
            self.in_buf_size += self
                .serial_stream
                .as_mut()
                .ok_or_else(|| format!("No serial stream"))?
                .read(&mut self.in_buf[self.in_buf_size..])
                .await
                .map_err(|e| format!("Unable to read on serial stream {:?}", e))?;

            // Try decoding
            let mut slip_decoder = serial_line_ip::Decoder::new();
            let (total_decoded, _out_slice, end) = slip_decoder
                .decode(&self.in_buf[..self.in_buf_size], response)
                .map_err(|e| format!("Unable to decode response: {:?}", e))?;

            if end {
                return Ok(total_decoded);
            }
        }
    }
    
}


impl std::default::Default for PiochaWorld {
    fn default() -> Self {
        
        let usb_s = UsbSettings::new().set_vendor(0x16c0).set_model(0x05E1);
        let serial_s = SerialSettings::new().set_port_name_from_usb_settings(&usb_s).unwrap();

        PiochaWorld {
            usb_settings: usb_s,
            serial_settings: serial_s,
            serial_stream: None,
            in_buf: [0u8; 512],
            in_buf_size: 0,
        }
    }
}
