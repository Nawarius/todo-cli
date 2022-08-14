use std::{env, fs, io};
use std::collections::HashMap;
use fltk::{app, prelude::*, window::Window, button::Button, frame, group, input};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Todo {
    list_map: Vec<String>
}
impl Todo {
    fn new () -> Result<Todo, io::Error> {
        let file = fs::OpenOptions::new().write(true).create(true).read(true).open("db.json")?;

        match serde_json::from_reader(file) {
            Ok(list_map) => {
                //let list_map: HashMap<String, bool> = list_map;
                //let list_map: Vec<String> = list_map.iter().map(|(key, val)| format!("{} -> {}", key, val)).collect();
                Ok(Todo { list_map })
            },
            Err(e) if e.is_eof() => Ok(Todo { list_map: vec![] }),
            Err(e) => panic!("An error occurred: {}", e)
        }
    }

    fn add_task (&mut self, key: String) -> Option<()> {
        if let Some(_) = self.list_map.iter().find(|el| el.contains(&key)) { None } 
        else {
            let task = format!("{} -> {}", key, true);
            self.list_map.push(task);
            Some(())
        }
    }

    fn save (&self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new().write(true).create(true).read(true).open("db.json")?;
        serde_json::to_writer_pretty(f, &self.list_map)?;
        Ok(())
    }

    fn done_task (&mut self, key: &String) {
        if let Some(task) = self.list_map.iter_mut().find(|el| el.contains(key)) {
            println!("Success done -->{}<--", key);
            *task = format!("{} -> false", key);
            println!("{:?}", self.list_map);
            self.save();
        } 
    }

    fn reset_task (&mut self, key: &String) {
        if let Some(task) = self.list_map.iter_mut().find(|el| el.contains(key)) {
            println!("Success reset -->{}<--", key);
            *task = format!("{} -> true", key);
            println!("{:?}", self.list_map);
            self.save();
        } 
    }
}
static mut todo: Todo = Todo { list_map: vec![]};

fn main() {
    unsafe { todo = Todo::new().unwrap(); }

    let app = app::App::default();
    let mut wind = Window::new(100, 100, 800, 600, "Hello from rust");
    let mut _frame = frame::Frame::default();

    let mut offset = 40;

unsafe {
    for task in todo.list_map.iter() {
        let task = task.split(" -> ").collect::<Vec<&str>>()[0];

        let flex = group::Flex::default().with_size(800, 30).with_pos(0, offset).row();

        let inp = input::Input::default().with_size(300, 50).set_value(task);
        let mut reset_btn = Button::default().with_size(45, 50).with_label("Reset");
        let mut done_btn = Button::default().with_label("Done");
        let mut remove_btn = Button::default().with_size(45, 50).with_label("Remove");

        done_btn.set_callback(|_| { 
            todo.done_task(&task.to_string());
        });
        reset_btn.set_callback(|_| { 
            todo.reset_task(&task.to_string());
        });

        flex.end();

        offset += 50;
    }
}
    
    wind.end();
    wind.show();
    
    

    app.run().unwrap();
    
    println!("End")
}
