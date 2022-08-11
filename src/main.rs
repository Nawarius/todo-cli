use std::io::Read;
use std::{env, fs, io};
use std::collections::HashMap;
use std::str::FromStr;

struct Todo {
    list_map: HashMap<String, bool>
}
impl Todo {
    fn new () -> Result<Todo, io::Error> {
        let mut file = fs::OpenOptions::new().write(true).create(true).read(true).open("db.txt")?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;
        let list_map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.split(" -> ").collect::<Vec<&str>>())
            .map(|vec| (String::from(vec[0]), bool::from_str(vec[1]).unwrap()))
            .collect();

        Ok(Todo { list_map })
    }

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

    let mut todo = Todo::new().unwrap();

    if action == "add" {
        todo.add(item);

        match todo.save() {
            Ok(_) => println!("Success save"),
            Err(_) => println!("Error save")
        };

    };
    
}
