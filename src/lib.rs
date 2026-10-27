//! A simple, Python-like input macro for Rust.
use std::io::{self, BufRead};

/// Reads a line from a given `BufRead` instance.
///
/// Trims trailing '\r' and '\n' characters while preserving internal whitespace
pub fn read_line_from<R: BufRead>(mut reader: R) -> io::Result<String> {
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    if buffer.ends_with('\n') {
        buffer.pop();
        if buffer.ends_with('\r') {
            buffer.pop();
        }
    }

    Ok(buffer)
}

/// Reads a line from standard input or a custom reader.
///
/// Can be called without arguments (`input!()`), with custom reader syntax
/// (`input!(from: reader)`, `input!(from: reader, "Prompt: {} ", value)`), or with formatting arguments
/// identical to `print!`: (`input!("Prompt: {} ", value)`).
///
/// Returns a `std::io::Result<String>` containing the trimmed user input.
///
/// # Examples
///
/// Standard `stdin`:
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
///
/// Custom reader:
/// ```
/// use pyinput::input;
/// fn main() {
///     let mut c = std::io::Cursor::new("foo\n");
///     let name = input!(from: &mut c);
///     assert!(matches!(name.as_deref(), Ok("foo")))
/// }
/// ```
#[macro_export]
macro_rules! input {
    (@read $reader:expr, $prompt_fn:expr) => {{
        let mut f = || -> std::io::Result<String> {
            use std::io::Write;
            $prompt_fn();
            std::io::stdout().flush()?;
            $crate::read_line_from($reader)
        };
        f()
    }};

    (from: $reader: expr, $($arg:tt)+) => {
        $crate::input!(@read $reader, || print!($($arg)+))
    };

    (from: $reader:expr $(,)?) => {
        $crate::input!(@read $reader, || ())
    };

    ($($arg:tt)+) => {
        $crate::input!(from: &mut std::io::stdin().lock(), $($arg)+)
    };

    () => {
        $crate::input!(from: &mut std::io::stdin().lock())
    };
}
