//! A simple, Python-like input macro for Rust.

/// Reads a line from standard input.
///
/// Can be called without arguments (`input!()`) or with formatting arguments
/// identical to `print!`: (`input!("Prompt: {} ", value)`) to display a prompt first.
///
/// Returns a `std::io::Result<String>` containing the trimmed user input.
///
/// # Examples
///
/// ```no_run
/// use pyinput::input;
/// use std::error::Error;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let name = input!("Enter your name: ")?;
///     let age: i32 = input!("Enter your age, {name}: ")?.parse()?;
///     println!("Hello {name}, you are {age}!");
///     Ok(())
/// }
/// ```
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
