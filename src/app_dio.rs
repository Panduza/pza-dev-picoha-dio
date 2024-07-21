// Print debug support
use crate::print_debug_message;
use core::fmt::Write;

// USB Communications Class Device support
use usbd_serial::SerialPort;

/// Application Digital I/O
pub struct AppDio {
    // ...
}

impl AppDio {
    pub fn new() -> Self {
        AppDio {
            // ...
        }
    }

    pub fn process_incoming_data(
        &mut self,
        serial: &mut SerialPort<rp2040_hal::usb::UsbBus>,
        data: &[u8],
    ) {
        print_debug_message!("Received data: {:?}", data);

        // ...
    }
}
