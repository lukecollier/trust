use crate::data::{Session, Window, Pane};
use crate::tmux::*;

pub trait TmuxIO {
    fn unsafe_run(&self, target: &mut Target) -> Result<(), &str>;
}

impl TmuxIO for Session {
    fn unsafe_run(&self, target: &mut Target) -> Result<(), &str> {
        let default_window = Window::from(String::from("default"));
        let first_window_name = &self.windows.first()
            .unwrap_or(&default_window).name;
        create_session(&self.name, &first_window_name);
        let mut new_target = target.clone();
        let name = new_target.push(&self.name);
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
        if self.panes.len() > 1 {
            for n in 1..self.panes.len() {
                let pane_id = n.to_string();
                let mut pane_target = name.clone();
                let other_name = pane_target.push(&pane_id);
                split_window(&other_name.to_string(), Direction::Horizontal);
                self.panes[n].unsafe_run(name).unwrap();
            }
        } else if self.panes.len() == 1 {
            self.panes[0].unsafe_run(name).unwrap();
        }
        Ok(())
    }
}

impl TmuxIO for Pane {
    fn unsafe_run(&self, target: &mut Target) -> Result<(), &str> {
        Ok(())
    }
}

// impl Runner {
//     pub fn from(sessions: Vec<Session>) -> Runner {
//         Runner { sessions }
//     }

//     pub fn run(&self) -> () {
//         for session in &self.sessions {
//             if has_session(&session.name) {
//                 switch_client(&session.name);
//             } else {
//                 create_session(&session.name);
//                 let mut first = true;
//                 for window in &session.windows {
//                     if first {
//                         first = false;
//                         rename_window(&format!("{}:^", session.name), &window.name);
//                     } else {
//                         new_window(&session.name, &window.name);
//                         if window.panes.len() > 1 as usize {
//                             first = true;
//                             let mut pane_number = 0;
//                             for pane in &window.panes {
//                                 pane_number = pane_number+1;
//                                 let pane_id = pane_number.to_string();
//                                 if first {
//                                     first = false;
//                                 } else {
//                                     let target = Target::from_session_window(
//                                         &session.name, 
//                                         &window.name).to_string();
//                                     let dir = Direction::Horizontal;
//                                     split_window(&target, dir);
//                                 }
//                                 for _ in &pane.panes {
//                                     if first {
//                                         first = false;
//                                     } else {
//                                         let target = Target::from(
//                                             &session.name, 
//                                             &window.name, 
//                                             &pane_id).to_string();
//                                         println!("adding to {}", target);
//                                         let dir = Direction::Horizontal;
//                                         split_window(&target, dir);
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

// // 1. loop sessions, create_session() 
// // 2. loop windows, new window using prev session name
// // 3. ???
// // 4. profit
