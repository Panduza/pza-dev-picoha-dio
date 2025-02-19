#[cfg(any(feature = "uart0_debug"))]
use bsp::hal::pac;
#[cfg(any(feature = "uart0_debug"))]
use rp2040_hal::uart::UartPeripheral;
#[cfg(any(feature = "uart0_debug"))]
use rp_pico as bsp;
#[cfg(any(feature = "uart0_debug"))]
use rp_pico::hal::gpio::Pin;

#[cfg(not(feature = "uart0_debug"))]
pub type UartType = ();

/// Type alias for the UART peripheral 0
#[cfg(any(feature = "uart0_debug"))]
pub type UartType = UartPeripheral<
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

#[cfg(any(feature = "uart0_debug"))]
pub fn uart_debug_print(debug_uart: &Option<UartType>, data: &[u8]) {
    if let Some(uart) = debug_uart.as_ref() {
        uart.write_full_blocking(data);
    }
}

#[macro_export]
#[cfg(not(any(feature = "uart0_debug")))]
macro_rules! print_debug_message {
    ($uart:expr, $fmt:expr) => {{}};
    ($uart:expr, $fmt:expr, $arg0:expr) => {{
        let _ = ($uart, $arg0);
        ()
    }};
    ($uart:expr, $fmt:expr, $arg0:expr, $arg1:expr) => {{
        let _ = ($uart, $arg0, $arg1);
        ()
    }};
}

#[macro_export]
#[cfg(any(feature = "uart0_debug"))]
macro_rules! print_debug_message {
    ($uart:expr, $fmt:expr) => {{
        crate::uart_debug_print($uart, $fmt);
    }};
    ($uart:expr, $fmt:expr, $arg0:expr) => {{
        let mut debug_message = heapless::String::<512>::new();
        writeln!(&mut debug_message, $fmt, $arg0).unwrap();
        crate::uart_debug_print($uart, debug_message.as_bytes());
    }};
    ($uart:expr, $fmt:expr, $arg0:expr, $arg1:expr) => {{
        let mut debug_message = heapless::String::<512>::new();
        writeln!(&mut debug_message, $fmt, $arg0, $arg1).unwrap();
        crate::uart_debug_print($uart, debug_message.as_bytes());
    }};
}
