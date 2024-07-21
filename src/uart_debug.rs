use rp_pico as bsp;

use bsp::hal::pac;
use rp2040_hal::uart::UartPeripheral;
use rp_pico::hal::gpio::Pin;

/// Type alias for the UART peripheral 0
type UartType = UartPeripheral<
    rp2040_hal::uart::Enabled,
    pac::UART0,
    (
        Pin<
            rp2040_hal::gpio::bank0::Gpio0,
            rp2040_hal::gpio::FunctionUart,
            rp2040_hal::gpio::PullDown,
        >,
        Pin<
            rp2040_hal::gpio::bank0::Gpio1,
            rp2040_hal::gpio::FunctionUart,
            rp2040_hal::gpio::PullDown,
        >,
    ),
>;

static mut DEBUG_UART: Option<UartType> = None;

pub fn uart_debug_init(uart: UartType) {
    unsafe {
        DEBUG_UART = Some(uart);
    }
}

pub fn uart_debug_print(data: &[u8]) {
    unsafe {
        if let Some(uart) = DEBUG_UART.as_ref() {
            uart.write_full_blocking(data);
        }
    }
}

#[macro_export]
macro_rules! print_debug_message {
    ($fmt:expr) => {{
        uart_debug_print($fmt);
    }};
    ($fmt:expr, $arg0:expr) => {{
        let mut debug_message = heapless::String::<512>::new();
        writeln!(&mut debug_message, $fmt, $arg0).unwrap();
        crate::uart_debug_print(debug_message.as_bytes());
    }};
    ($fmt:expr, $arg0:expr, $arg1:expr) => {{
        let mut debug_message = String::<512>::new();
        writeln!(&mut debug_message, $fmt, $arg0, $arg1).unwrap();
        crate::uart_debug_print(debug_message.as_bytes());
    }};
}
