use crate::data::{Session, Window, Pane, Layout};
use crate::tmux::*;

pub trait TmuxIO {
    fn unsafe_run(&self, target: &mut Target) -> Result<(), &str>;
}

impl TmuxIO for Session {
    fn unsafe_run(&self, target: &mut Target) -> Result<(), &str> {
        let default_window = Window::from(String::from("default"), Layout::EvenVertical);
        let first_window_name = &self.windows.first()
            .unwrap_or(&default_window).name;
        create_session(&self.name, &first_window_name);
        let mut new_target = target.clone();
        let name = new_target.push(&self.name);
        if !self.windows.is_empty() {
            let mut first_window_name = target.clone();
            first_window_name.push(&self.name);
            first_window_name.push(&self.windows.first().unwrap().name);
            let window_commands = &self.windows.first().unwrap().commands;
            for command in window_commands {
                send_command(&first_window_name.to_string(), &command).expect("failed to send");
                send_command(&first_window_name.to_string(), "Enter").expect("failed to send");
            }
        }
        if self.windows.len() > 1 {
            for n in 1..self.windows.len() {
                self.windows[n].unsafe_run(name).unwrap();
            }
        }
        Ok(())
    }
}

impl TmuxIO for Window {
    fn unsafe_run(&self, target: &mut Target) -> Result<(), &str> {
        new_window(&target.to_string(), &self.name);
        let mut new_target = target.clone();
        let name = new_target.push(&self.name);
        if self.panes.len() > 0 {
            let mut pane_target = name.clone();
            let other_name = pane_target.push("1");
            self.panes[0].unsafe_run(other_name).unwrap();
        }
        if self.panes.len() > 1 {
            for n in 1..self.panes.len() {
                let pane_id = (n + 1).to_string();
                let split_pane_id = (n).to_string();
                let mut pane_target = name.clone();
                pane_target.push(&split_pane_id);
                split_window(&pane_target.to_string());
                let mut new_pane_target = name.clone();
                new_pane_target.push(&pane_id);
                self.panes[n].unsafe_run(&mut new_pane_target).unwrap();
            }
        }
        for command in &self.commands {
            send_command(&name.to_string(), &command).expect("failed to send");
            send_command(&name.to_string(), "Enter").expect("failed to send");
        }
        select_layout(&name.to_string(), &self.layout.to_string());
        Ok(())
    }
}

impl TmuxIO for Pane {
    fn unsafe_run(&self, target: &mut Target) -> Result<(), &str> {
        for command in &self.commands {
            send_command(&target.to_string(), &command).expect("failed to send");
            send_command(&target.to_string(), "Enter").expect("failed to send");
        }
        Ok(())
    }
}

