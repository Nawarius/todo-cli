use std::{fs, io};
use crate::ToDoApp;

pub trait TodoDataWorker {
    fn read_todo_from_db () -> Result<Vec<String>, io::Error> {
        let file = fs::OpenOptions::new().write(true).create(true).read(true).open("db.json").unwrap();

        match serde_json::from_reader(file) {
            Ok(list_map) => Ok( list_map ),
            Err(e) if e.is_eof() => Ok( vec![] ),
            Err(e) => panic!("An error occurred: {}", e)
        }
    }

    fn save (&self) -> Result<(), Box<dyn std::error::Error>>;

    fn mark_task (&mut self, key: &String, msg: &str);

    fn add_task (&mut self, task_name: &str) ;

    fn remove_task (&mut self, task_name: &str);
}

impl TodoDataWorker for ToDoApp {
    
    fn save (&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::write("db.json", serde_json::to_string_pretty(&self.list_map).unwrap())?;
        Ok(())
    }

    fn mark_task (&mut self, key: &String, msg: &str) {
        if let Some(task) = self.list_map.iter_mut().find(|el| el.contains(key)) {
            *task = format!("{} -> {}", key, msg);
            self.save();
        } 
    }

    fn add_task (&mut self, task_name: &str) {
        if let None = self.list_map.iter().find(|el| el.contains(task_name)) { 
            let task = format!("{} -> Not Done", task_name);
            self.list_map.push(task);
            self.save();
        } 
    }

    fn remove_task (&mut self, task_name: &str) {
        if let Some(index) = self.list_map.iter().position(|el| el.contains(task_name)) {
            self.list_map.remove(index);
            self.save();
        }
    }
}