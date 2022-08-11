use std::{env, fs, io};
use std::collections::HashMap;

struct Todo {
    list_map: HashMap<String, bool>
}
impl Todo {
    fn new () -> Result<Todo, io::Error> {
        let file = fs::OpenOptions::new().write(true).create(true).read(true).open("db.json")?;
        //let mut content = String::new();

        // file.read_to_string(&mut content)?;
        // let list_map: HashMap<String, bool> = content
        //     .lines()
        //     .map(|line| line.split(" -> ").collect::<Vec<&str>>())
        //     .map(|vec| (String::from(vec[0]), bool::from_str(vec[1]).unwrap()))
        //     .collect();

        match serde_json::from_reader(file) {
            Ok(list_map) => Ok(Todo { list_map }),
            Err(e) if e.is_eof() => Ok(Todo { list_map: HashMap::new() }),
            Err(e) => panic!("An error occurred: {}", e)
        }
        
    }

    fn add_task (&mut self, key: String) {
        self.list_map.insert(key, true);
    }
    fn save (&self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new().write(true).create(true).open("db.json")?;
        
        serde_json::to_writer_pretty(f, &self.list_map)?;
        Ok(())

        // let mut result = String::new();

        // for (key, val) in self.list_map.iter() {
        //     let record = format!("{} -> {}\n", key, val);
        //     result.push_str(&record);
        // }
        // fs::write("db.txt", result)
    }
    fn done_task (&mut self, key: &String) -> Option<()> {
        match self.list_map.get_mut(key) {
            Some(val) => Some(*val = false),
            None => None
        }
    }
}
fn main() {
    let action = env::args().nth(1).expect("Need some action");
    let item = env::args().nth(2).expect("Need some item");

    let mut todo = Todo::new().unwrap();

    if action == "add" {
        todo.add_task(item.clone());

        match todo.save() {
            Ok(_) => println!("Success save"),
            Err(_) => println!("Error save")
        };

    } else if action == "done" {
        match todo.done_task(&item) {
            Some(_) => match todo.save() {
                Ok(_) => println!("Success save"),
                Err(_) => println!("Error save")
            },
            None => println!("The task completed unsuccessful")
        };
    }
    
}
