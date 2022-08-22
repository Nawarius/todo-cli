mod todo_data_worker;
mod todo_visualizer;

use crate::todo_data_worker::TodoDataWorker;
use crate::todo_visualizer::TodoVisualizer;

use fltk::{app, prelude::*, window::Window, frame::Frame, input::Input};

#[derive(Copy, Clone)]
pub enum TaskMessage {
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

impl ToDoApp {
    
    pub fn new () -> Self {
        let list_map = ToDoApp::read_todo_from_db().unwrap();
        
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

                        self.add_input.take_focus();
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








