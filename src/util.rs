use std::io;
use std::process::exit;
#[macro_export]
macro_rules! wprintln {
    ($fmt:expr) => (print!(concat!($fmt, "\r\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\r\n"), $($arg)*));
}

/**
 * A function to trim trailing '\n' and '\r\n'
*/
pub fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

pub fn get_tipp() -> bool {
    let mut input = String::new();
    wprintln!("Willst du einen Tipp haben? [j/n]");
    if io::stdin().read_line(&mut input).is_err() {
        wprintln!("Ich erwarte eine Eingabe");
        exit(1);
    }
    trim_newline(&mut input);
    input.as_str().eq("j")
}
