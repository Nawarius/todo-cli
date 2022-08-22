//#![windows_subsystem = "windows"]
use to_do::ToDoApp;

fn main() {
    unsafe {
        let mut a = ToDoApp::new();
        a.run();
    }
}
