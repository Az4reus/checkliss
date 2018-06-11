use std::io::Error;
use std::process::Command;
use std::process::Output;

use item::Item;
use io::save_string_locally;
use opts::Config;

fn compose_tex(root: Item) -> Result<String, Error> {
    let mut content = String::new();
    content.push_str(&generate_header());

    let tex = root.children_tex();
    content.push_str(&tex);

    content.push_str(&generate_footer());
    Ok(content)
}

pub fn compile_tex(root: Item, config: &Config) -> Result<(), Error> {
    let tex = compose_tex(root).expect("bad input");

    match save_string_locally(tex, "temp.tex") {
        Ok(()) => {
            let output: Output = Command::new("xelatex").arg("./temp.tex").output().unwrap();

            match output.status.success() {
                true => {
                    if config.keep_tex == false {
                        Command::new("rm")
                            .arg("./temp.tex")
                            .output()
                            .expect("Couldn't delete temp file.");
                    }
                    Ok(())
                }
                false => {
                    println!("{:?}", output);
                    panic!("compilation failed")
                }
            }
        }
        Err(e) => Err(e),
    }
}

fn generate_header() -> String {
    r#"
%!TEX TS-program = xetex
%!TEX encoding = UTF-8 Unicode

\documentclass[a4paper]{article}

\usepackage{fontspec}
\setmainfont{Roboto Slab Regular}

\usepackage{enumitem,amssymb}
\newlist{todolist}{itemize}{2}
\setlist[todolist]{label=$\square$}

\begin{document}
\Large
\begin{todolist}
"#.to_owned()
}

fn generate_footer() -> String {
    r#"
\end{todolist}
\end{document}"#.to_owned()
}
