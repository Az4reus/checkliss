use std::io::Error;
use std::env;

use item::Item;

pub fn compose_tex(root: Item) -> Result<String, Error> {
    let mut content = String::new();
    content.push_str(&generate_header());

    let tex = root.to_tex();
    content.push_str(&tex);

    content.push_str(&generate_footer());
    Ok(content)
}

pub fn compile_tex(tex: String) -> Result<(), Error> {
    unimplemented!();

    let pwd = env::current_dir();
}

fn generate_header() -> String {
r#"\documentclass[a4paper]{article}

\usepackage[sfdefault]{roboto}
\usepackage[utf8]{inputenc}
\usepackage[T1]{fontenc}

\geometry{
    body={6.5in, 8.5in},
    left=1.0in,
    top=1.25in
}
\begin{document}
"#.to_owned()
}

fn generate_footer() -> String {
r#"
\end{document}"#.to_owned()
}