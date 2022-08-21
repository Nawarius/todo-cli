use std::{fs, io};
use fltk::{app, prelude::*, window::Window, button::Button, frame::Frame, group::Flex, input::Input};
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


#[derive(Copy, Clone)]
enum TaskMessage {
    Done(&'static str),
    Reset(&'static str),
    Add(),
    Remove(&'static str)
}
struct ToDoApp {
    app: app::App,
    window: Window,
    receiver: app::Receiver<TaskMessage>,
    sender: app::Sender<TaskMessage>,
    list_map: Vec<String>,
    frame_map: Vec<(String, Frame)>,
    add_input: Input
}

static mut TaskVec: Vec<String> = vec![];

impl ToDoApp {
    fn read_todo_drom_db () -> Result<Vec<String>, io::Error> {
        let file = fs::OpenOptions::new().write(true).create(true).read(true).open("db.json").unwrap();

        match serde_json::from_reader(file) {
            Ok(list_map) => Ok( list_map ),
            Err(e) if e.is_eof() => Ok( vec![] ),
            Err(e) => panic!("An error occurred: {}", e)
        }
    }

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
        } 
    }

    fn new () -> Self {
        let list_map = ToDoApp::read_todo_drom_db().unwrap();
        
        let app = app::App::default();
        let (sender, receiver) = app::channel();
        let window = Window::default().with_size(800, 600).with_label("App");

        Self {
            app,
            window,
            sender,
            receiver,
            list_map,
            frame_map: vec![],
            add_input: Input::default(),
        }
    }

    fn clear_window (&mut self) {
        self.frame_map = vec![];

        self.window.clear();
        self.window.redraw();
    }

    unsafe fn view (&mut self) {
        self.clear_window();

        TaskVec = self.list_map.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        let mut offset = 40;

        self.window.begin();

        let flex_add = Flex::default().with_size(800, 30).with_pos(0, 0).row();

            let add_input = Input::default().with_size(200, 30);
            self.add_input = add_input;
            
            let mut add_btn = Button::default().with_size(100, 30).with_label("Add task");
            add_btn.emit(self.sender, TaskMessage::Add());

        flex_add.end();

        for task in TaskVec.iter() {
            let vec: Vec<&str> = task.split(" -> ").collect();
            let task = vec[0];
            let res = vec[1];

            let flex = Flex::default().with_size(800, 30).with_pos(0, offset).row();

                let task_name = Frame::default().with_label(task);
                let task_res = Frame::default().with_label(res);

                self.frame_map.push((task.to_string(), task_res));

                let mut reset_btn = Button::default().with_size(45, 50).with_label("Reset task");
                let mut done_btn = Button::default().with_label("Done task");
                let mut remove_btn = Button::default().with_size(45, 50).with_label("Remove");

                done_btn.emit(self.sender, TaskMessage::Done(task));
                reset_btn.emit(self.sender, TaskMessage::Reset(task));
                remove_btn.emit(self.sender, TaskMessage::Remove(task));

            flex.end();

            offset += 50;
        }
        self.window.end();
        self.window.show();

    }

    unsafe fn run(&mut self) {
        while self.app.wait() {
            if let Some(msg) = self.receiver.recv() {
                match msg {
                    TaskMessage::Done(key) => {
                        if let Some((task_name, frame)) = self.frame_map.iter_mut().find(|(name, _)| name.contains(key)) {
                            frame.set_label("Done");

                            let task_name = task_name.to_string();
                            self.mark_task(&task_name, "Done");
                        } 
                    },
                    TaskMessage::Reset(key) => {
                        if let Some((task_name, frame)) = self.frame_map.iter_mut().find(|(name, _)| name.contains(key)) {
                            frame.set_label("Not Done");

                            let task_name = task_name.to_string();
                            self.mark_task(&task_name, "Not Done");
                        } 
                    },
                    TaskMessage::Add() => {
                        let task_name = self.add_input.value();
                        self.add_task(&task_name);

                        self.add_input.set_value("");
                        self.view();
                    },
                    TaskMessage::Remove(key) => {
                        println!("{key}");
                    },
                }
            }
        }
    }
}

fn main() {
    unsafe {
        let mut a = ToDoApp::new();
        a.view();
        a.run();
    }
    
    // APP.lock().unwrap().view();
    // APP.lock().unwrap().run();
//     unsafe { todo = Todo::new().unwrap(); }

//     let app = app::App::default();
//     let mut wind = Window::new(100, 100, 800, 600, "Hello from rust");
//     let mut _frame = frame::Frame::default();

//     let mut offset = 40;

// unsafe {
//     for task in todo.list_map.iter() {
//         let vec: Vec<&str> = task.split(" -> ").collect();
//         let task = vec[0];
//         let res = vec[1];

//         let flex = group::Flex::default().with_size(800, 30).with_pos(0, offset).row();

//         let task_name = input::Input::default().with_size(300, 50).set_value(task);
//         let mut task_res = input::Input::default();
//         task_res.with_size(300, 50).set_value(res);
//         let mut reset_btn = Button::default().with_size(45, 50).with_label("Reset task");
//         let mut done_btn = Button::default().with_label("Done task");
//         let mut remove_btn = Button::default().with_size(45, 50).with_label("Remove");

//         done_btn.set_callback(|_| { 
//             //btn.emit(sender, msg)
//             todo.done_task(&task.to_string());
//             //task_res.set_value("Hmm");
//         });
        
//         reset_btn.set_callback(|_| { 
//             todo.reset_task(&task.to_string());
//         });

//         flex.end();

//         offset += 50;
//     }
// }
    
//     wind.end();
//     wind.show();
    
    

//     app.run().unwrap();
    
//     println!("End")
}
