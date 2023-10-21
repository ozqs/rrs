use std::fmt;
use std::process::exit;

pub fn fatal(s: &str) -> ! {
    println!("Fatal: {s}");
    std::process::exit(1)
}

pub trait Fatal<T, E: fmt::Display> {
    fn fatal(self, msg: &str) -> T;
}

impl<T, E: fmt::Display> Fatal<T, E> for Result<T, E> {
    fn fatal(self, msg: &str) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                println!("Fatal: {}\n错误信息: {}", msg, e.to_string());
                exit(1);
            }
        }
    }
}
