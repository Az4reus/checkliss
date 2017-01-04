use std::process::Command;

mod repl;
mod tex;


fn main() {
    match has_xetex() {
        false => println!("Please install XeTeX to use this tool."),
        true => {
            let items = repl::launch_repl();
            let tex = tex::compose_tex(items.unwrap()).unwrap();
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
        assert!(has_xetex())
    }
}
