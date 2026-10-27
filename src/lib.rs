#[macro_export]
macro_rules! input {
    () => {{
        let __input_f: fn() -> std::io::Result<String> = (|| {
            let mut __input_buf = String::new();
            std::io::stdin().read_line(&mut __input_buf)?;
            Ok(__input_buf.trim().to_string())
        });
        __input_f()
    }};
    ($($arg:tt)+) => {{
        let __input_f  = (|| {
            use std::io::Write;
            print!($($arg)+);
            std::io::stdout().flush()?;
            $crate::input!()
        });
        __input_f()
    }};
}
