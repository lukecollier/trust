use crate::data::{Session};
use crate::tmux::{create_session, new_window, has_session};

pub struct Runner {
    sessions: Vec<Session>
}

impl Runner {
    pub fn from(sessions: Vec<Session>) -> Runner {
        Runner { sessions: sessions }
    }

    pub fn run(&self) -> () {
        for session in &self.sessions {
            if has_session(&session.name) {
                // todo attach session
            } else {
                create_session(&session.name);
                for window in &session.windows {
                    new_window(&session.name, &window.name);
                }
            }
        }
    }
}

// 1. loop sessions, create_session() 
// 2. loop windows, new window using prev session name
// 3. ???
// 4. profit
