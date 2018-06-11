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

    match tex::compile_tex(root_node.unwrap(), &config) {
        Ok(()) => println!("Success! File generated."),
        Err(e) => println!("Compilation failed, error: {:?}", e),
    }
}

pub fn has_xetex() -> bool {
    match Command::new("xetex").arg("-v").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_sanity() {
        assert!(1 + 1 == 2);
    }
}
