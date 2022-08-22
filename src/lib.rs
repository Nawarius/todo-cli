use std::{fs, io};
use fltk::{app, prelude::*, window::Window, button::Button, frame::Frame, group::Flex, input::Input};
use serde_json;

#[derive(Copy, Clone)]
enum TaskMessage {
    Done(&'static str),
    Reset(&'static str),
    Add(),
    Remove(&'static str)
}
pub struct ToDoApp {
    app: app::App,
    window: Window,
    receiver: app::Receiver<TaskMessage>,
    sender: app::Sender<TaskMessage>,
    list_map: Vec<String>,
    frame_map: Vec<(String, Frame)>,
    add_input: Input
}

static mut TaskVec: Vec<String> = vec![];

trait TodoDataWorker {
    fn read_todo_drom_db () -> Result<Vec<String>, io::Error>;
    fn save (&self) -> Result<(), Box<dyn std::error::Error>>;
    fn add_task (&mut self, task_name: &str);
    fn remove_task (&mut self, task_name: &str);
    fn mark_task (&mut self, key: &String, msg: &str);
}

trait TodoVisualizer {
    fn clear_window (&mut self);
    unsafe fn view (&mut self);
}

impl TodoDataWorker for ToDoApp {
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
impl TodoVisualizer for ToDoApp {
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

            let mut add_input = Input::default().with_size(200, 30);
            add_input.emit(self.sender, TaskMessage::Add());
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
}

impl ToDoApp {
    
    pub fn new () -> Self {
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

    pub unsafe fn run(&mut self) {
        self.view();

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
                    TaskMessage::Remove(task_name) => {
                        self.remove_task(task_name);
                        self.view();
                    },
                }
            }
        }
    }
}
