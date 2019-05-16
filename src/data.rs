#[derive(Debug,Clone)]
pub struct Session {
    windows: Vec<Window>,
    name: String,
}
impl Session {
    pub fn from(name: String) -> Session {
        Session { windows: Vec::new(), name: name }
    }

    pub fn push_all(&mut self, windows: Vec<Window>) {
        for window in windows {
            self.windows.push(window);
        }
    }
}

#[derive(Debug,Clone)]
pub struct Window {
    panes: Vec<Pane>,
    name: String,
}
impl Window {
    pub fn from(name: String) -> Window {
        Window { panes: Vec::new(), name: name }
    }

    pub fn push_all(&mut self, panes: Vec<Pane>) {
        for pane in panes {
            self.panes.push(pane);
        }
    }
}

#[derive(Debug,Clone)]
pub struct Pane {
    panes: Vec<Pane>,
    commands: Vec<String>,
    name: String
}

impl Pane {
    pub fn from(name: String) -> Pane {
        Pane { panes: Vec::new(), name: name, commands: Vec::new() }
    }

    pub fn push_all(&mut self, panes: Vec<Pane>) {
        for pane in panes {
            self.panes.push(pane);
        }
    }

    pub fn commands(&mut self, commands: Vec<String>) {
        for command in commands {
            self.commands.push(command);
        }
    }
}
