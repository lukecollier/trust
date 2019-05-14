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

fn handle_event<'a>(event: Event<'a>) -> () {
    let mut depth = 0;
    let mut previous_depth = 0;
    let mut children: Vec<Pane> = Vec::new();
    match event {
        Event::Start(ref _e) => {
            previous_depth = depth.clone();
            depth += 1;
        },
        Event::Text(ref _e) => {
            ()
        },
        Event::End(ref e) => {
            depth -= 1;
            let name = String::from(str::from_utf8(e.name()).unwrap());
            if previous_depth <= depth {
                children.push(Pane::new(name));
            } else {
                let mut parent = Pane::new(name);
                let children_add = children.split_off(0);
                parent.push_all(children_add);
                children.push(parent);
            }
        },
        _ => {()}
    }
}

fn main() {
    let path = Path::new("./src/resources/test.xml");
    let mut reader = Reader::from_file(path).expect("failed to read");
    reader.trim_text(true);

    let mut depth = 0;
    let mut previous_depth = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    let mut children: Vec<Pane> = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref _e)) => {
                previous_depth = depth.clone();
                depth += 1;
            },
            Ok(Event::End(ref e)) => { 
                depth -= 1;
                let name = String::from(str::from_utf8(e.name()).unwrap());
                if previous_depth <= depth {
                    // println!("panel: {} at ({},{})", str::from_utf8(e.name()).unwrap(), depth, previous_depth);
                    children.push(Pane::new(name));
                } else {
                    let mut parent = Pane::new(name);
                    // println!("creating panel {} at depth {} with children {:?}", str::from_utf8(e.name()).unwrap(), depth, children);
                    let children_add = children.split_off(0);
                    parent.push_all(children_add);
                    children.push(parent);
                }
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()), // todo convert text to commands seperated by new lines
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => ()
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    println!("root elements {:?}", children);
}
