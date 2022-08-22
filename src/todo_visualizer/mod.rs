use fltk::{prelude::*, text::{TextEditor, TextBuffer}, button::Button, frame::Frame, group::{Flex, Scroll}, input::Input};
use crate::{ToDoApp, TaskMessage};

static mut TASK_VEC: Vec<String> = vec![];

pub trait TodoVisualizer {
    fn clear_window (&mut self);
    unsafe fn view (&mut self);
}

impl TodoVisualizer for ToDoApp {

    fn clear_window (&mut self) {
        self.frame_map = vec![];

        self.window.clear();
        self.window.redraw();
    }

    unsafe fn view (&mut self) {
        self.clear_window();

        TASK_VEC = self.list_map.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        let mut offset = 40;

        self.window.begin();

        let scroll_bar = Scroll::default().with_size(800, 600);

        let flex_add = Flex::default().with_size(780, 30).with_pos(0, 0).row();

            let mut add_input = Input::default().with_size(200, 30);
            add_input.emit(self.sender, TaskMessage::Add());
            self.add_input = add_input;
            
            let mut add_btn = Button::default().with_size(100, 30).with_label("Add task");
            add_btn.emit(self.sender, TaskMessage::Add());

        flex_add.end();

        for task in TASK_VEC.iter() {
            let vec: Vec<&str> = task.split(" -> ").collect();
            let task = vec[0];
            let res = vec[1];

            let flex = Flex::default().with_size(780, 30).with_pos(0, offset).row();

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

        scroll_bar.end();

        self.window.end();
        self.window.show();

    }
}

