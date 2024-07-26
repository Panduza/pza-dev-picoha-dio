#[cfg(any(feature = "uart0_debug"))]
use bsp::hal::pac;
#[cfg(any(feature = "uart0_debug"))]
use rp2040_hal::uart::UartPeripheral;
#[cfg(any(feature = "uart0_debug"))]
use rp_pico as bsp;
#[cfg(any(feature = "uart0_debug"))]
use rp_pico::hal::gpio::Pin;

/// Type alias for the UART peripheral 0
#[cfg(any(feature = "uart0_debug"))]
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

#[cfg(any(feature = "uart0_debug"))]
static mut DEBUG_UART: Option<UartType> = None;

#[cfg(any(feature = "uart0_debug"))]
pub fn uart_debug_init(uart: UartType) {
    unsafe {
        DEBUG_UART = Some(uart);
    }
}

#[cfg(any(feature = "uart0_debug"))]
pub fn uart_debug_print(data: &[u8]) {
    unsafe {
        if let Some(uart) = DEBUG_UART.as_ref() {
            uart.write_full_blocking(data);
        }
    }
}

#[macro_export]
#[cfg(not(any(feature = "uart0_debug")))]
macro_rules! print_debug_message {
    ($fmt:expr) => {{}};
    ($fmt:expr, $arg0:expr) => {{}};
    ($fmt:expr, $arg0:expr, $arg1:expr) => {{}};
}

#[macro_export]
#[cfg(any(feature = "uart0_debug"))]
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
        let mut debug_message = heapless::String::<512>::new();
        writeln!(&mut debug_message, $fmt, $arg0, $arg1).unwrap();
        crate::uart_debug_print(debug_message.as_bytes());
    }};
}
