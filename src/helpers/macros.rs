// -------------
// Custom Macros
// -------------

#[macro_export]
macro_rules! std_error_exit {
    ($message:expr) => {{
        eprintln!("{}", $message);
        ::std::process::exit(1);
    }};
}
