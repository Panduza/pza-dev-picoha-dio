use embassy_rp::peripherals;
use embassy_rp::uart;

pub type DebugUart<'d> = uart::Uart<'d, peripherals::UART0, uart::Blocking>;

static mut NONE: Option<DebugUart> = None;
static mut UART_DEBUG: &mut Option<DebugUart> = unsafe { &mut NONE };

#[cfg(any(feature = "uart0_debug"))]
pub fn uart_debug_init(uart: &'static mut Option<DebugUart>) {
    unsafe {
        UART_DEBUG = uart;
    }
}

#[cfg(any(feature = "uart0_debug"))]
pub fn uart_debug_print(data: &[u8]) {
    unsafe {
        if let Some(uart) = UART_DEBUG.as_mut() {
            let _ = uart.blocking_write(data);
        }
    }
}

#[macro_export]
#[cfg(not(any(feature = "uart0_debug")))]
macro_rules! print_debug_message {
    ($fmt:expr) => {{}};
    ($fmt:expr, $arg0:expr) => {{
        let _ = ($arg0);
        ()
    }};
    ($fmt:expr, $arg0:expr, $arg1:expr) => {{
        let _ = ($arg0, $arg1);
        ()
    }};
}

#[macro_export]
#[cfg(any(feature = "uart0_debug"))]
macro_rules! print_debug_message {
    ($fmt:expr) => {{
        crate::uart_debug_print($fmt);
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
