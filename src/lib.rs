mod todo_data_worker;
mod todo_visualizer;

use crate::todo_data_worker::TodoDataWorker;
use crate::todo_visualizer::TodoVisualizer;

use fltk::{app, prelude::*, window::Window, frame::Frame, input::Input};
use fltk_theme::{WidgetTheme, ThemeType};

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
    todo_list: Vec<String>,
    add_input: Input
}

impl ToDoApp {
    
    pub fn new () -> Self {
        let todo_list = ToDoApp::read_todo_from_db().unwrap();
        
        let app = app::App::default();

        let widget_theme = WidgetTheme::new(ThemeType::Dark);
        widget_theme.apply();

        let (sender, receiver) = app::channel();
        let window = Window::default().with_size(800, 600).with_label("ToDo List");

        Self {
            app,
            window,
            sender,
            receiver,
            todo_list,
            add_input: Input::default(),
        }
    }

    pub unsafe fn run(&mut self) {
        self.view();

        while self.app.wait() {
            if let Some(msg) = self.receiver.recv() {
                match msg {
                    TaskMessage::Done(task_name) => {
                        self.mark_task(task_name, "Done");
                        self.view();
                    },
                    TaskMessage::Reset(task_name) => {
                        self.mark_task(task_name, "Not Done");
                        self.view();
                    },
                    TaskMessage::Remove(task_name) => {
                        self.remove_task(task_name);
                        self.view();
                    },
                    TaskMessage::Add() => {
                        let task_name = self.add_input.value();
                        self.add_task(&task_name);

                        self.add_input.set_value("");
                        self.view();

                        self.add_input.take_focus();
                    }
                }
            }
        }
    }
}








