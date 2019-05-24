use crate::data::{Session};
use crate::tmux::*;

pub struct Runner {
    sessions: Vec<Session>
}

impl Runner {
    pub fn from(sessions: Vec<Session>) -> Runner {
        Runner { sessions }
    }

    pub fn run(&self) -> () {
        for session in &self.sessions {
            if has_session(&session.name) {
                // todo attach session
            } else {
                create_session(&session.name);
                let mut first = true;
                for window in &session.windows {
                    if first {
                        first = false;
                        rename_window(&format!("{}:^", session.name), &window.name);
                    } else {
                        new_window(&session.name, &window.name);
                        if window.panes.len() > 1 as usize {
                            first = true;
                            for _ in &window.panes {
                                if first {
                                    first = false;
                                } else {
                                    let target = Target::from_session_window(&session.name, &window.name).to_string();
                                    let dir = Direction::Horizontal;
                                    split_window(&target, dir);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// 1. loop sessions, create_session() 
// 2. loop windows, new window using prev session name
// 3. ???
// 4. profit
