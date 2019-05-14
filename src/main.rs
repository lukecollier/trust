use quick_xml::Reader;
use std::path::Path;
use quick_xml::events::Event;
use std::str;


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
    children: Vec<Pane>
}

impl State {
    fn new() -> State {
        State { depth: 0, prev_depth: 0, children: Vec::new() }
    }
}

fn handle_event<'a>(event: Event<'a>, state: &mut State) -> () {
    match event {
        Event::Start(ref _e) => {
            state.prev_depth = state.depth.clone();
            state.depth += 1;
        },
        Event::Text(ref _e) => {
            ()
        },
        Event::End(ref e) => {
            state.depth -= 1;
            let name = String::from(str::from_utf8(e.name()).unwrap());
            if state.prev_depth <= state.depth {
                state.children.push(Pane::new(name));
            } else {
                let mut parent = Pane::new(name);
                let children_add = state.children.split_off(0);
                parent.push_all(children_add);
                state.children.push(parent);
            }
        },
        _ => {()}
    }
}

fn main() {
    let path = Path::new("./src/resources/test.xml");
    let mut reader = Reader::from_file(path).expect("failed to read");
    reader.trim_text(true);

    let mut buf = Vec::new();

    let mut state = State::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(event) => handle_event(event, &mut state), 
            Err(error) => panic!("Error at position {}: {:?}", reader.buffer_position(), error),
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    println!("work {:?}", state.children);

}
