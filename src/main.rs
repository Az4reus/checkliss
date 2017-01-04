use std::process::Command;

mod repl;
mod tex;
mod io;
mod item;

fn main() {
    match has_xetex() {
        false => println!("Please install XeTeX to use this tool."),
        true => {
            let item = repl::launch_repl().expect("wtf?");
            match tex::compile_tex(item) {
                Ok(_) => println!("All went well, your file is somewhere in $PWD here."),
                Err(e) => println!("Shit crashed and burned, here's why: {}", e),
            }
        }
    }
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
    fn test_playground() {
        use item::Item;
        use tex::compile_tex;

        let test_node = Item {
            title: "Test!".to_owned(),
            children: Vec::new(),
            indent_level: 0,
        };

        compile_tex(test_node);
    }
}
