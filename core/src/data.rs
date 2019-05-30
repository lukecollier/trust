#[derive(Debug, PartialEq, Eq)]
pub struct Session {
    pub windows: Vec<Window>,
    pub name: String,
    pub commands: Vec<String>,
}
impl Session {
    pub fn from(name: String) -> Session {
        Session { windows: Vec::new(), name, commands: Vec::new() }
    }

    pub fn push_all(&mut self, windows: Vec<Window>) {
        for window in windows {
            self.windows.push(window);
        }
    }

    pub fn commands(&mut self, commands: Vec<String>) {
        for command in commands {
            self.commands.push(command);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Layout {
    EvenHorizontal,
    EvenVertical,
    MainHorizontal,
    MainVertical,
    Tiled,
}
impl Layout {
    pub fn from(string: &str) -> Result<Layout, String> {
        match string {
            "even-horizontal" => Ok(Layout::EvenHorizontal),
            "even-vertical" => Ok(Layout::EvenVertical),
            "main-horizontal" => Ok(Layout::MainHorizontal),
            "main-vertical" => Ok(Layout::MainVertical),
            "tiled" => Ok(Layout::Tiled),
            _ => Err(format!("{} is not a valid layout", string)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Layout::EvenHorizontal => String::from("even-horizontal"),
            Layout::EvenVertical => String::from("even-vertical"),
            Layout::MainHorizontal => String::from("main-horizontal"),
            Layout::MainVertical => String::from("main-vertical"),
            Layout::Tiled => String::from("tiled"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Window {
    pub panes: Vec<Pane>,
    pub commands: Vec<String>,
    pub name: String,
    pub layout: Layout
}
impl Window {
    pub fn from(name: String, layout: Layout) -> Window {
        Window { panes: Vec::new(), name, layout, commands: Vec::new() }
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

#[derive(Debug, PartialEq, Eq)]
pub struct Pane {
    pub commands: Vec<String>,
    pub name: String
}

impl Pane {
    pub fn from(name: String) -> Pane {
        Pane { name, commands: Vec::new() }
    }

    pub fn commands(&mut self, commands: Vec<String>) {
        for command in commands {
            self.commands.push(command);
        }
    }

}
// todo define iterator for pane that get's all children 
