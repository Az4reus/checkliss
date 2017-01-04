
pub struct Item {
    pub title: String,
    pub children: Option<Vec<Item>>,
    pub indent_level: i32,
}

impl Item {
    pub fn to_tex(&self) -> String {
        unimplemented!();
    }
}