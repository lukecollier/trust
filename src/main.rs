use quick_xml::Reader;
use quick_xml::events::Event;

use std::path::Path;
use std::str;
use std::fmt;


struct Session<'a> {
    windows: Vec<&'a Window<'a>>,
    reference: [u8],
}

struct Window<'a> {
    panes: Vec<&'a Pane>,
    title: str,
}

#[derive(Debug,Clone)]
struct Pane {
    panes: Vec<Pane>,
    commands: Vec<String>,
    name: String
}
impl Pane {
    fn new(name: String) -> Pane {
        Pane { panes: Vec::new(), name: name, commands: Vec::new() }
    }

    fn push_all(&mut self, panes: Vec<Pane>) {
        for pane in panes {
            self.panes.push(pane);
        }
    }

    fn commands(&mut self, commands: Vec<String>) {
        for command in commands {
            self.commands.push(command);
        }
    }
}

impl fmt::Display for Pane {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "({}, {})", self.x, self.y);
        // the output should be as a tree that scales depending on size of console
        unimplemented!()
    }
}

#[derive(Debug)]
enum TmuxType {
    Session,
    Pane,
    Window
}

fn get_tmux_type(depth: &u8) -> TmuxType {
    match depth {
        // 0 => TmuxType::Session,
        // 1 => TmuxType::Window,
        // 2 => TmuxType::Pane,
        _ => TmuxType::Pane,
    }
}

pub struct Parser {
    depth: usize,
    prev_depth: usize,
    children: Vec<Pane>,
    root: Vec<Pane>,
    commands_hierarchy: Vec<Vec<String>>
}

impl Parser {
    fn new() -> Parser {
        Parser { 
            depth: 0, 
            prev_depth: 0, 
            children: Vec::new(), 
            root: Vec::new(),
            commands_hierarchy: Vec::new()
        }
    }

    fn is_root(depth: usize) -> bool { depth == 0 }

    fn handle_event<'a, B: std::io::BufRead>(&mut self, event: Event<'a>, reader: &mut Reader<B>) -> () {
        match event {
            Event::Start(ref _e) => {
                self.prev_depth = self.depth.clone();
                self.depth += 1;
                self.commands_hierarchy.resize(self.depth + 1, Vec::new());
            },
            Event::Text(ref e) => {
                self.commands_hierarchy[self.depth].push(e.unescape_and_decode(&reader).unwrap());
            },
            Event::End(ref e) => {
                self.depth -= 1;
                let name = String::from(str::from_utf8(e.name()).unwrap());
                let mut pane = Pane::new(name);
                pane.commands(self.commands_hierarchy.clone().into_iter().flatten().collect());
                if self.prev_depth <= self.depth {
                    self.children.push(pane);
                } else {
                    let children_add = self.children.split_off(0);
                    pane.push_all(children_add);
                    if Parser::is_root(self.depth) {
                        self.root.push(pane);
                    } else {
                        self.children.push(pane);
                    }
                }
                self.commands_hierarchy.resize(self.depth + 1, Vec::new());
            },
            _ => ()
        }
    }

    pub fn parse<B: std::io::BufRead>(reader: &mut Reader<B>) -> () {
        let mut buf = Vec::new();

        let mut state = Parser::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Eof) => break, // break loop when event at end of file
                Ok(event) => Parser::handle_event(&mut state, event, reader), 
                Err(error) => panic!("Error at position {}: {:?}", reader.buffer_position(), error),
            }
            buf.clear(); // keep memory usage low
        }

        let mut i = 0;
        for x in &state.root {
            i += 1;
            println!("{}: {:?}", i, x);
        }
    }
}


fn main() {
    let path = Path::new("./src/resources/test.xml");
    let mut reader = Reader::from_file(path).expect("failed to read");
    reader.trim_text(true);
    Parser::parse(&mut reader);
}
