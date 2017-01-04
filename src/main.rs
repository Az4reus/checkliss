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
            let tex = tex::compose_tex(item).unwrap();
            match tex::compile_tex(tex) {
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
        use std::env::current_dir;

        let pwd = current_dir().unwrap();
        println!("{}", pwd.to_str().unwrap());
    }
}
