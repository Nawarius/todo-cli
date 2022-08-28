use std::{fs, io};
use crate::ToDoApp;

pub trait TodoDataWorker {
    fn read_todo_from_db () -> Result<Vec<String>, io::Error> {
        let file = fs::OpenOptions::new().write(true).create(true).read(true).open("db.json").unwrap();

        match serde_json::from_reader(file) {
            Ok(todo_list) => Ok( todo_list ),
            Err(e) if e.is_eof() => Ok( vec![] ),
            Err(e) => panic!("An error occurred: {}", e)
        }
    }

    fn save (&self);

    fn change_task_state (&mut self, task_name: &str, task_state: &str);

    fn add_task (&mut self, task_name: &str) ;

    fn remove_task (&mut self, task_name: &str, task_state: &str);
}

impl TodoDataWorker for ToDoApp {
    
    fn save (&self) {
        std::fs::write(
            "db.json", 
            serde_json::to_string_pretty(&self.todo_list).unwrap()
        ).unwrap();
    }

    fn change_task_state (&mut self, task_name: &str, task_state: &str) {
        if let Some(task) = self.todo_list.iter_mut().find(|el| el.contains(task_name)) {
            *task = format!("{} -> {}", task_name, task_state);
            self.save();
        } 
    }

    fn add_task (&mut self, task_name: &str) {
        let task = format!("{} -> Not Done", task_name);

        if let None = self.todo_list.iter().find(|el| **el == task) { 
            self.todo_list.push(task);
            self.save();
        } 
    }

    fn remove_task (&mut self, task_name: &str, task_state: &str) {
        let task = format!("{} -> {}", task_name, task_state);

        if let Some(index) = self.todo_list.iter().position(|el| *el == task) {
            self.todo_list.remove(index);
            self.save();
        }
    }
}