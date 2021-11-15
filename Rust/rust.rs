// To run
// rustc rust.rs
// ./rust
use std::env;
use std::io::{self, Write};
use std::io::prelude::*;

fn main() -> io::Result<()>{
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    let args: Vec<String> = env::args().collect();
    let arg =  &args[1];
    let n:i32 = arg.trim().parse().expect("NaN: Argument is not a number");

    let f = "Fizz";
    let b = "Buzz";
    let fb = "FizzBuzz";

    for i in 1..n+1 {
        if (i % 3 == 0) && (i % 5 == 0) {
            io::stdout().write_all(fb.as_bytes())?;
        } else if i % 3 == 0 {
            io::stdout().write_all(f.as_bytes())?;
        } else if i % 5 == 0 {
            io::stdout().write_all(b.as_bytes())?;
        } else {
            io::stdout().write_all(i.to_string().as_bytes())?;
        }
    }
    Ok(())
}