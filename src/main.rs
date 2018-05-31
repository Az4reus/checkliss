use std::process::Command;
use std::process;

extern crate clap;

mod tex;
mod io;
mod item;
mod parser;
mod opts;
mod list;

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

    #[test]
    fn test_integration() {
        // if it doesn't die somewhere, we're good.

        use std::fs::remove_file;
        use std::env::current_dir;

        use item::Item;
        use tex::compile_tex;

        let test_node = Item::new("test parent".to_owned()).add_child(Item::new("test child".to_owned()));

        compile_tex(test_node);

        let mut aux = current_dir().unwrap();
        let mut log = current_dir().unwrap();
        let mut pdf = current_dir().unwrap();
        let mut tex = current_dir().unwrap();

        aux.push("temp.aux");
        log.push("temp.log");
        pdf.push("temp.pdf");
        tex.push("temp.tex");

        remove_file(aux).unwrap();
        remove_file(log).unwrap();
        remove_file(pdf).unwrap();
        remove_file(tex).unwrap();
    }
}
