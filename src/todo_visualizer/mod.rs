use fltk::{prelude::*, *, button::Button, frame::Frame, group::{Flex, Scroll}, input::Input};
use crate::{ToDoApp, TaskMessage};
use fltk_theme::{widget_themes};

static mut TASK_VEC: Vec<String> = vec![];

pub trait TodoVisualizer {
    fn clear_window (&mut self);
    unsafe fn view (&mut self);
}

impl TodoVisualizer for ToDoApp {

    fn clear_window (&mut self) {
        self.window.clear();
        self.window.redraw();
    }

    unsafe fn view (&mut self) {
        self.clear_window();

        TASK_VEC = self.todo_list.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        let mut offset = 40;

        self.window.begin();

        let scroll_bar = Scroll::default().with_size(800, 600);

        let flex_add = Flex::default().with_size(780, 30).with_pos(0, 0).row();
        
            let mut add_input = Input::default();
            add_input.take_focus().unwrap();
            add_input.set_trigger(enums::CallbackTrigger::EnterKey);
            add_input.emit(self.sender, TaskMessage::Add());
            
            let mut add_btn = Button::default().with_label("Add task");
            add_btn.emit(self.sender, TaskMessage::Add());
            add_btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);

        flex_add.end();

        for task in TASK_VEC.iter() {
            let vec: Vec<&str> = task.split(" -> ").collect();
            let task = vec[0];
            let current_state = vec[1];

            let flex = Flex::default().with_size(780, 30).with_pos(0, offset).row();

                Frame::default().with_label(task);
                Frame::default().with_label(current_state);
            
                let mut reset_btn = Button::default().with_label("Reset task");
                reset_btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
                let mut done_btn = Button::default().with_label("Done task");
                done_btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
                let mut remove_btn = Button::default().with_label("Remove");
                remove_btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);

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

