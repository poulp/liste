pub struct Item {
    pub link: String,
    pub title: String,
    pub description: String,

    pub is_read: bool
}

impl Item {
    pub fn new(link: String, title: String, description: String, is_read: bool) -> Item {
        Item {
            link: link,
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