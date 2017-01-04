
#[derive(Clone)]
pub struct Item {
    pub title: String,
    pub children: Vec<Item>,
    pub indent_level: i32,
}

impl Item {
    pub fn to_tex(&self) -> String {
        match self.children.is_empty() {
            true => format!("\\item{{ {} }} \n", self.title),
            false => {
                let mut res = format!("{{\\texbf \\item{{ {} }} }} \n", self.title);
                let children_tex = self.children.clone().into_iter()
                    .map(|a| a.to_tex())
                    .fold(res.clone(), |mut acc, item| {
                        acc.push_str(&item);
                        acc
                    });

                res.push_str(&children_tex);
                res
            }
        }
    }

    pub fn new() -> Item {
        Item{
            title: String::new(),
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
        let testchild1 = Item::new();
        let testchild2 = Item::new();

        let test = Item::new().add_child(testchild1).add_child(testchild2);
        assert!(test.children.len() == 2);
    }
}
