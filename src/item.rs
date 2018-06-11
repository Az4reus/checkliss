#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub title: String,
    pub children: Vec<Item>,
}

impl Item {
    pub fn to_tex(&self) -> String {
        match self.children.is_empty() {
            true => format!("\\item{{{}}} \n", self.title),
            false => {
                let mut res = format!("\\item{{\\textbf{{{}}}}}\n", self.title);
                res.push_str("\\begin{todolist}\n");

                for child in self.children.clone() {
                    let tex = child.to_tex();
                    res.push_str(&tex);
                    res.push_str("\n");
                }

                res.push_str("\\end{todolist}\n");
                res
            }
        }
    }

    /// This function is used for not generating an empty top-level list when one item is used as
    /// root of the tree. 
    pub fn children_tex(&self) -> String {
        let mut buffer = String::new();
        for child in self.children.clone() {
            let tex = child.to_tex();
            buffer.push_str(&tex);
            buffer.push_str("\n")
        }

        buffer
    }

    pub fn new(name: String) -> Item {
        Item {
            title: name.trim().to_owned(),
            children: Vec::new(),
        }
    }

    pub fn set_children(self, children: Vec<Item>) -> Item {
        Item {
            title: self.title, 
            children: children,
        }
    }

    pub fn add_child(self, child: Item) -> Item {
        let mut tmp = self.children;
        tmp.push(child);

        Item {
            title: self.title,
            children: tmp,
        }
    }

    pub fn set_title(&mut self, new_title: String) -> Item {
        Item {
            title: new_title,
            children: self.children.clone(),
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

        let test = Item::new("testParent".to_owned())
            .add_child(testchild1)
            .add_child(testchild2);
        assert!(test.children.len() == 2);
    }
}
