use core::get_session_names;
use core::tmux::*;

fn main() {
    let session_names = get_session_names();
    let session_states = get_sessions_state();
    let attached_session_names: Vec<_> = session_states
        .iter()
        .filter(|state| match state {
            SessionState::Attached(_) => true,
            SessionState::Detached(_) => false,
        })
        .map(|state| match state {
            SessionState::Attached(name) => name,
            SessionState::Detached(name) => name,
        })
        .collect();

    for name in &session_names {
        if attached_session_names.contains(&name) {
            println!("{}", name);
        } else {
            println!("{}", name);
        }
    }
}

enum SessionState {
    Attached(String),
    Detached(String),
}

fn get_sessions_state() -> Vec<SessionState> {
    let sessions =
        list_sessions("#{session_name}:#{session_attached}").expect("could not list sessions");
    sessions
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(':').collect();
            if split[1].parse::<i32>().unwrap() > 0 {
                SessionState::Attached(String::from(split[0]))
            } else {
                SessionState::Detached(String::from(split[0]))
            }
        })
        .collect()
}
