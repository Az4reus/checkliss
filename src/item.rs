
#[derive(Clone)]
pub struct Item {
    pub title: String,
    pub children: Vec<Item>,
    pub indent_level: i32,
}

impl Item {
    pub fn to_tex(&self) -> String {
        match self.children.is_empty() {
            true => format!("\\item{{{}}} \n", self.title),
            false => {
                let mut res = format!("\\item{{\\textbf{{{}}}}}\n", self.title);
                res.push_str("\\begin{itemize} %parent \n");

                for child in self.children.clone() {
                    let tex = child.to_tex();
                    res.push_str(&tex);
                    res.push_str("\n");
                }

                res.push_str("\\end{itemize} %parent\n");
                res
            }
        }
    }

    pub fn new(name: String) -> Item {
        Item{
            title: name,
            children: Vec::new(),
            indent_level: 0
        }
    }

    pub fn add_child(self, child: Item) -> Item {
        let mut tmp = self.children;
        tmp.push(child);

        Item {
            title: self.title,
            children: tmp,
            indent_level: self.indent_level,
        }
    }

    pub fn set_title(&mut self, new_title: String) -> Item {
        Item {
            title: new_title,
            children: self.children.clone(),
            indent_level: self.indent_level,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_builders() {
        let testchild1 = Item::new("testhcild1".to_owned());
        let testchild2 = Item::new("testchild2".to_owned());

        let test = Item::new("testParent".to_owned()).add_child(testchild1).add_child(testchild2);
        assert!(test.children.len() == 2);
    }
}
