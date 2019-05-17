#[derive(Debug)]
pub struct Session {
    pub windows: Vec<Window>,
    pub name: String,
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

#[derive(Debug)]
pub struct Window {
    pub panes: Vec<Pane>,
    pub name: String,
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

#[derive(Debug)]
pub struct Pane {
    pub panes: Vec<Pane>,
    pub commands: Vec<String>,
    pub name: String
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
