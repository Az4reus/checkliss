use std::io::Error;

pub struct Item {
    pub title: String,
    pub children: Option<Vec<Item>>,
}

pub fn launch_repl() -> Result<Vec<Item>, Error> {
    unimplemented!();
}