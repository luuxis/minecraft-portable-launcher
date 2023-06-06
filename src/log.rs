#[macro_export]
macro_rules! log {
    ($severity:literal, $filename:expr, $message: literal $(, $arg:expr)*) => {
        if cfg!(debug_assertions) {
            println!(concat!($severity, " {}: ", $message), $filename$(, $arg)*);
        }
    };
    ($severity:literal, $message: literal $(, $arg:expr)*) => {
        if cfg!(debug_assertions) {
            println!(concat!($severity, ": ", $message)$(, $arg)*);
        }
    };
    () => {

    };
}