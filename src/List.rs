use item::Item;

#[derive(Clone)]
pub struct List {
    name: String, 
    items: Vec<Item>,
}

impl List {
    pub fn new(items: Vec<Item>) -> List {
        List {
            name: String::new(),
            items: items,
        }
    }

    pub fn set_name(self, name: String) -> List {
        List {
            name: name, 
            items: self.items,
        }
    }
}
