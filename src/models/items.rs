pub struct Item {
    pub title: String,
    pub description: String,

    pub is_read: bool
}

impl Item {
    pub fn new(title: String, description: String, is_read: bool) -> Item {
        Item {
            title: title,
            description: description,
            is_read: is_read
        }
    }
}

pub struct ListItems {
    pub items: Vec<Item>
}

impl ListItems {
    pub fn new() -> ListItems {
        ListItems {
            items: vec![]
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}