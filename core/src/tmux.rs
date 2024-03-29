use std::process::{Command, Output};
use std::str;

// todo: tmux commands don't need to be attached too, we can set up the env
pub fn kill_session(target: &str) -> Result<String, String> {
    let output = Command::new("tmux")
        .arg("kill-session")
        .arg("-t")
        .arg(target)
        .output()
        .expect("failed to execute process");
    handle_output(output)
}

pub fn create_session(name: &str, window_name: &str) -> Result<String, String> {
    let output = Command::new("tmux")
        .arg("new-session")
        .arg("-d")
        .arg("-s")
        .arg(name)
        .arg("-n")
        .arg(window_name)
        .output()
        .expect("failed to execute process");
    handle_output(output)
}

pub fn list_sessions(format: &str) -> Result<String, String> {
    let output = Command::new("tmux")
        .arg("list-session")
        .arg("-F")
        .arg(format)
        .output()
        .expect("failed to execute process");
    handle_output(output)
}

pub fn new_window(target: &str, name: &str) -> Result<String, String> {
    let output = Command::new("tmux")
        .arg("new-window")
        .arg("-t")
        .arg(target)
        .arg("-n")
        .arg(name)
        .output()
        .expect("failed to execute process");
    handle_output(output)
}

pub fn select_layout(target: &str, layout: &str) -> Result<String, String> {
    let output = Command::new("tmux")
        .arg("select-layout")
        .arg("-t")
        .arg(target)
        .arg(layout)
        .output()
        .expect("failed to execute process");
    handle_output(output)
}

pub fn split_window(target: &str) -> Result<String, String> {
    let output = Command::new("tmux")
        .arg("split-window")
        .arg("-t")
        .arg(target)
        .output()
        .expect("failed to execute process");

    handle_output(output)
}

fn handle_output(output: Output) -> Result<String, String> {
    if output.status.success() {
        let out = str::from_utf8(&output.stdout).unwrap();
        Ok(String::from(out.trim()))
    } else {
        let err = str::from_utf8(&output.stderr).unwrap();
        Err(String::from(err.trim()))
    }
}

pub fn send_command(target: &str, command: &str) -> Result<String, String> {
    let output = Command::new("tmux")
        .arg("send-keys")
        .arg("-t")
        .arg(target)
        .arg(command)
        .output()
        .expect("failed to execute process");
    handle_output(output)
}

pub fn version() -> Result<String, String> {
    let output = Command::new("tmux")
        .arg("-V")
        .output()
        .expect("failed to execute process");
    handle_output(output)
}

pub fn has_session(name: &str) -> bool {
    Command::new("tmux")
        .arg("has-session")
        .arg("-t")
        .arg(name)
        .output()
        .expect("failed to execute process")
        .status
        .success()
}

#[derive(Debug, Clone)]
pub struct Target {
    session: Option<String>,
    window: Option<String>,
    pane: Option<String>,
}

impl Target {
    pub fn new() -> Target {
        Target {
            session: None,
            window: None,
            pane: None,
        }
    }

    pub fn push(&mut self, name: &str) -> &mut Self {
        if self.session == None {
            self.session = Some(String::from(name));
        } else if self.window == None {
            self.window = Some(String::from(name));
        } else if self.pane == None {
            self.pane = Some(String::from(name));
        } else {
            panic!("Attempted to push to full target");
        }
        self
    }

    pub fn from(session: &str, window: &str, pane: &str) -> Target {
        Target {
            session: Some(String::from(session)),
            window: Some(String::from(window)),
            pane: Some(String::from(pane)),
        }
    }

    pub fn to_string(&self) -> String {
        match (&self.session, &self.window, &self.pane) {
            (None, None, None) => String::new(),
            (Some(session), None, None) => session.to_string(),
            (Some(session), Some(window), None) => format!("{}:{}", session, window),
            (Some(session), Some(window), Some(pane)) => format!("{}:{}.{}", session, window, pane),
            (None, Some(window), None) => format!(":{}", window),
            (None, Some(window), Some(pane)) => format!(":{}.{}", window, pane),
            (None, None, Some(pane)) => format!(".{}", pane),
            (Some(session), None, Some(pane)) => format!("{}.{}", session, pane),
        }
    }
}
