#![windows_subsystem = "windows"]

mod state;
mod window;

use druid::{AppLauncher, LocalizedString, WindowDesc};

use crate::state::State;

pub fn main() {
    let window = WindowDesc::new(window::Window::build())
        .resizable(false)
        .title(LocalizedString::new("calculator").with_placeholder("Calculator"));
    let calc_state = State::default();

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(calc_state)
        .expect("launch failed");
}
