use std::io; //导入输入输出模块

//定义结构体
struct TodoItem {
    id: u32,
    title: String,
    completed: bool,
}


struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { items: vec![] }
    }

    fn add_item(&mut self, title: String) {
        let new_item = TodoItem {
            id: self.items.len() as u32 + 1,
            title,
            completed: false,
        };
        self.items.push(new_item);
    }

    fn get_item(&self, id: u32) -> Option<&TodoItem> {
        self.items.iter().find(|item| item.id == id)
    }

    fn update_item(&mut self, id: u32, new_title: Option<String>, new_completed: Option<bool>) {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            if let Some(title) = new_title {
                item.title = title;
            }
            if let Some(completed) = new_completed {
                item.completed = completed;
            }
        }
    }

    fn delete_item(&mut self, id: u32) {
        self.items.retain(|item| item.id != id);
    }

    fn display_items(&self) {
        for item in &self.items {
            println!("ID: {}, Title: {}, Completed: {}", item.id, item.title, item.completed);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    loop {
        println!("Choose an option:");
        println!("1. Add item");
        println!("2. View item");
        println!("3. Update item");
        println!("4. Delete item");
        println!("5. Display all items");
        println!("6. Exit");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            1 => {
                println!("Enter title for new item:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                todo_list.add_item(title.trim().to_string());
            },
            2 => {
                println!("Enter ID of item to view:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                if let Some(item) = todo_list.get_item(id) {
                    println!("ID: {}, Title: {}, Completed: {}", item.id, item.title, item.completed);
                } else {
                    println!("Item not found.");
                }
            },
            3 => {
                println!("Enter ID of item to update:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                println!("Enter new title (press enter to skip):");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                let title = if title.trim().is_empty() { None } else { Some(title.trim().to_string()) };

                println!("Is the item completed? (yes/no, press enter to skip):");
                let mut completed = String::new();
                io::stdin().read_line(&mut completed).expect("Failed to read line");
                let completed = match completed.trim() {
                    "yes" => Some(true),
                    "no" => Some(false),
                    _ => None,
                };

                todo_list.update_item(id, title, completed);
            },
            4 => {
                println!("Enter ID of item to delete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                todo_list.delete_item(id);
            },
            5 => {
                todo_list.display_items();
            },
            6 => break,
            _ => println!("Invalid option, try again."),
        }
    }
}

