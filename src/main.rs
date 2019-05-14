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
    name: String
}
impl Pane {
    fn new(name: String) -> Pane {
        Pane { panes: Vec::new(), name: name }
    }

    fn push_all(&mut self, panes: Vec<Pane>) {
        for pane in panes {
            self.panes.push(pane);
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

struct State {
    depth: u8,
    prev_depth: u8,
    children: Vec<Pane>,
    root: Vec<Pane>,
}

impl State {
    fn new() -> State {
        State { 
            depth: 0, 
            prev_depth: 0, 
            children: Vec::new(), 
            root: Vec::new() 
        }
    }
}

fn is_root(depth: u8) -> bool {
    depth == 0
}

fn handle_event<'a>(event: Event<'a>, state: &mut State) -> () {
    match event {
        Event::Start(ref e) => {
            println!("{} {} {}", str::from_utf8(e.name()).unwrap(), state.depth, state.prev_depth);
            state.prev_depth = state.depth.clone();
            state.depth += 1;
        },
        Event::Text(ref _e) => { () },
        Event::End(ref e) => {
            state.depth -= 1;
            println!("/{} {} {}", str::from_utf8(e.name()).unwrap(), state.depth, state.prev_depth);
            let name = String::from(str::from_utf8(e.name()).unwrap());
            if state.prev_depth <= state.depth {
                state.children.push(Pane::new(name));
            } else {
                let mut parent = Pane::new(name);
                let children_add = state.children.split_off(0);
                parent.push_all(children_add);
                if is_root(state.depth) {
                    state.root.push(parent);
                } else {
                    state.children.push(parent);
                }
            }
        },
        _ => {()}
    }
}

fn parse_into_roots<B: std::io::BufRead>(reader: &mut Reader<B>) -> () {
    let mut buf = Vec::new();
    let mut state = State::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Eof) => break, // break loop when event at end of file
            Ok(event) => handle_event(event, &mut state), 
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

fn main() {
    let path = Path::new("./src/resources/test.xml");
    let mut reader = Reader::from_file(path).expect("failed to read");
    reader.trim_text(true);
    parse_into_roots(&mut reader);
}
