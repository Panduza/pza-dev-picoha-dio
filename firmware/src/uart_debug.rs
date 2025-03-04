#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug {
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
#[cfg(debug_assertions)]
macro_rules! debug {
    ($fmt:expr) => {{
        log::debug!($fmt);
    }};
    ($fmt:expr, $arg0:expr) => {{
        log::debug!($fmt, $arg0);
    }};
    ($serial_debug:expr, $fmt:expr, $arg0:expr, $arg1:expr) => {{
        log::debug!($fmt, $arg0, $arg1);
    }};
}
