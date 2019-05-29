use quick_xml::Reader;
use quick_xml::events::Event;
use std::path::Path;
use std::str;

use crate::data::{Session, Window, Pane};

pub struct Parser {
    depth: usize,
    prev_depth: usize,
    sessions: Vec<Session>,
    windows: Vec<Window>,
    panes: Vec<Pane>,
    commands_hierarchy: Vec<Vec<String>>
}

impl Parser {
    fn new() -> Parser {
        Parser { 
            depth: 0, 
            prev_depth: 0, 
            panes: Vec::new(), 
            windows: Vec::new(), 
            sessions: Vec::new(), 
            commands_hierarchy: Vec::new()
        }
    }

    fn handle_child(&mut self, name: String) {
        match self.depth {
            0 => self.sessions.push(Session::from(name)),
            1 => self.windows.push(Window::from(name)),
            _ => {
                let mut pane = Pane::from(name);
                pane.commands(self.commands_hierarchy.clone().into_iter().flatten().collect());
                self.panes.push(pane);
            },
        }
    }

    fn handle_parent(&mut self, name: String) {
        match self.depth {
            0 => { 
                let mut session = Session::from(name);
                let windows_add = self.windows.split_off(0);
                session.push_all(windows_add);
                self.sessions.push(session);
            }, 
            1 => {
                let mut window = Window::from(name);
                let children = self.panes.split_off(0);
                window.push_all(children);
                self.windows.push(window);
            },
            _ => {
                let mut pane = Pane::from(name);
                let children = self.panes.split_off(0);
                pane.push_all(children);
                pane.commands(self.commands_hierarchy.clone().into_iter().flatten().collect());
                self.panes.push(pane);
            },
        }
    }

    fn is_on_parent(&self) -> bool {
        self.prev_depth <= self.depth
    }

    fn handle_event<'a, B: std::io::BufRead>(&mut self, event: Event<'a>, reader: &mut Reader<B>) {
        match event {
            Event::Start(ref _e) => {
                self.prev_depth = self.depth;
                self.depth += 1;
                self.commands_hierarchy.resize(self.depth + 1, Vec::new());
            },
            Event::Text(ref e) => {
                let decoded_command = e.unescape_and_decode(&reader).unwrap();
                for pane in &mut self.panes {
                    pane.commands.push(decoded_command.clone());
                };
                for window in &mut self.windows {
                    for pane in &mut window.panes {
                        pane.commands.push(decoded_command.clone());
                    }
                };
                self.commands_hierarchy[self.depth]
                    .push(e.unescape_and_decode(&reader).unwrap());
            },
            Event::End(ref e) => {
                self.depth -= 1;
                let name = String::from(str::from_utf8(e.name()).unwrap());
                if self.is_on_parent() {
                    self.handle_child(name);
                } else {
                    self.handle_parent(name);
                }
                self.commands_hierarchy.resize(self.depth + 1, Vec::new());
            },
            _ => ()
        }
    }

    pub fn parse<B: std::io::BufRead>(reader: &mut Reader<B>) -> Vec<Session> {
        let mut buf = Vec::new();
        let mut state = Parser::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Eof) => break, // break loop when event at end of file
                Ok(event) => Parser::handle_event(&mut state, event, reader), 
                Err(error) => panic!("Error at position {}: {:?}", reader.buffer_position(), error),
            }
            buf.clear();
        }
        state.sessions
    }

    pub fn from_file(path: &Path) -> Vec<Session> {
        let mut reader = Reader::from_file(path).expect("failed to read");
        reader.trim_text(true);
        Parser::parse(&mut reader)
    }

    #[allow(dead_code)]
    pub fn from_string(text: &str) -> Vec<Session> {
        let mut reader = Reader::from_str(text);
        reader.trim_text(true);
        Parser::parse(&mut reader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse {
        use super::*;
        #[test]
        fn test_empty_xml() {
            assert_eq!(Parser::from_string(""), Vec::new());
        }

        #[test]
        fn test_singular_xml() {
            assert_eq!(Parser::from_string("<base></base>"), vec![Session::from("base".to_string())]);
        }

        #[test]
        fn test_multiple_xml() {
            let expected =vec![
                Session::from("one".to_string()),
                Session::from("two".to_string()),
            ];
            assert_eq!(Parser::from_string("<one></one><two></two>"), expected);
        }
        #[test]
        fn test_can_have_retro_active_commands() {
            let mut session = Session::from("session".to_string());
            let mut window = Window::from("window".to_string());
            let mut pane = Pane::from("pane".to_string());

            pane.commands.push(String::from("command_one"));
            pane.commands.push(String::from("command_two"));
            window.push(pane);
            session.push(window);
            let expected =vec![
                session
            ];
            assert_eq!(Parser::from_string("<session>command_one\n<window><pane></pane></window>\ncommand_two\n</session>"), expected);
        }
    }

}
