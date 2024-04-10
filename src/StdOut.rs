use std::io::{self, Write};

/// Writes data of various types to standard output.
pub struct StdOut;

impl StdOut {
    /// Terminates the current line by printing the line-separator string.
    pub fn println() {
        println!();
    }

    /// Prints an object to this output stream and then terminates the line.
    pub fn print<T: std::fmt::Display>(x: T) {
        println!("{}", x);
    }

    /// Flushes standard output.
    pub fn flush() {
        io::stdout().flush().unwrap();
    }

    /// Prints a formatted string to standard output, using the specified format
    /// string and arguments, and then flushes standard output.
    pub fn printf(format: &str, args: &[&dyn std::fmt::Display]) {
        let mut output = format.to_string();
        for arg in args {
            output = output.replacen("{}", &format!("{}", arg), 1);
        }
        print!("{}", output);
        Self::flush();
    }
}

fn main() {
    // write to stdout
    StdOut::print("Test");
    StdOut::print(17);
    StdOut::print(true);
    StdOut::printf("{:.6}", &[&1.0 / 7.0]);
}
