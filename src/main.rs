use std::{fs, io};
use fltk::{app, prelude::*, window::Window, button::Button, frame, group, input};
use serde_json;

#[derive(Debug, Clone)]
struct Todo {
    list_map: Vec<String>
}

impl Todo {
    fn new () -> Result<Todo, io::Error> {
        let file = fs::OpenOptions::new().write(true).create(true).read(true).open("db.json")?;

        match serde_json::from_reader(file) {
            Ok(list_map) => Ok(Todo { list_map }),
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
        std::fs::write("db.json", serde_json::to_string_pretty(&self.list_map).unwrap())?;
        Ok(())
    }

    fn done_task (&mut self, key: &String) {
        if let Some(task) = self.list_map.iter_mut().find(|el| el.contains(key)) {
            println!("Success done -->{}<--", &key);
            *task = format!("{} -> Done", key);
            self.save();
        } 
    }

    fn reset_task (&mut self, key: &String) {
        if let Some(task) = self.list_map.iter_mut().find(|el| el.contains(key)) {
            println!("Success reset -->{}<--", &key);
            *task = format!("{} -> Not Done", key);
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
        let mut reset_btn = Button::default().with_size(45, 50).with_label("Reset task");
        let mut done_btn = Button::default().with_label("Done task");
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
