use std::process::Command;
use std::process;

extern crate clap;

mod tex;
mod io;
mod item;
mod parser;
mod opts;

fn main() {
    if !has_xetex() {
        println!("Please install XeTeX to use this tool.");
        process::exit(1)
    }

    let config: opts::Config = opts::parse();

    let root_node = parser::parse(&config);
}

pub fn has_xetex() -> bool {
    match Command::new("xetex")
        .arg("-v")
        .output() {
            Ok(output) => output.status.success(),
            Err(_) => false
        }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sanity() {
        assert!(1 + 1 == 2);
    }

    #[test]
    fn test_xetex_check() {
        // Yes, this is a test dependent on circumstances uncontrollable by the program itself. Sue me.
        assert!(has_xetex())
    }
}
