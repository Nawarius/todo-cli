use std::{env, fs, io};
use std::collections::HashMap;

struct Todo {
    list_map: HashMap<String, bool>
}
impl Todo {
    fn add (&mut self, key: String) {
        self.list_map.insert(key, true);
    }
    fn save (&self) -> Result<(), io::Error> {
        let mut result = String::new();

        for (key, val) in self.list_map.iter() {
            let record = format!("{} -> {}\n", key, val);
            result.push_str(&record);
        }
        fs::write("db.txt", result)
    }
}
fn main() {
    let action = env::args().nth(1).expect("Need some action");
    let item = env::args().nth(2).expect("Need some item");

    let mut todo = Todo { list_map: HashMap::new() };

    if action == "add" {
        todo.add(item);

        match todo.save() {
            Ok(_) => println!("Success save"),
            Err(_) => println!("Error save")
        }

    }
    
}
