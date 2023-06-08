use quick_xml::events::Event;
use quick_xml::Reader;
use std::path::Path;
use std::str;

use crate::data::{Layout, Pane, Session, Window};

// todo: The parser should just run the commands from the stack as it reads for effective
// efficiency, and super simplicity
// can emulate the act of a declararitve method by following the tree if it already exists and
// where it deviates BRINGING IT INLINE. At the pane level that would entail killing, then
// restarting the pain with whatever the command buffer is!
//
// Wowee I feel I've grown as an engineer revisiting this
// For perf reasons we should grab the current state of tmux in it's ENTIRETY and use that for
// comparisons as we stream the XML
pub struct Parser {
    layout: Option<Layout>,
    depth: usize,
    prev_depth: usize,
    sessions: Vec<Session>,
    windows: Vec<Window>,
    panes: Vec<Pane>,
    commands_hierarchy: Vec<Vec<String>>,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            layout: None,
            depth: 0,
            prev_depth: 0,
            panes: Vec::new(),
            windows: Vec::new(),
            sessions: Vec::new(),
            commands_hierarchy: Vec::new(),
        }
    }

    fn layout(&self) -> Layout {
        self.layout.clone().unwrap_or(Layout::EvenVertical)
    }

    fn handle_leaf(&mut self, name: String) {
        match self.depth {
            0 => {
                let mut session = Session::from(name);
                session.commands(
                    self.commands_hierarchy
                        .clone()
                        .into_iter()
                        .flatten()
                        .collect(),
                );
                self.sessions.push(session);
            }
            1 => {
                let mut window = Window::from(name, self.layout());
                window.commands(
                    self.commands_hierarchy
                        .clone()
                        .into_iter()
                        .flatten()
                        .collect(),
                );
                self.windows.push(window);
            }
            2 => {
                let mut pane = Pane::from(name);
                pane.commands(
                    self.commands_hierarchy
                        .clone()
                        .into_iter()
                        .flatten()
                        .collect(),
                );
                self.panes.push(pane);
            }
            _ => panic!("Cannot nest panes, support for this was dropped in alpha"),
        }
    }

    fn handle_node(&mut self, name: String) {
        match self.depth {
            0 => {
                let mut session = Session::from(name);
                let windows_add = self.windows.split_off(0);
                session.push_all(windows_add);
                session.commands(
                    self.commands_hierarchy
                        .clone()
                        .into_iter()
                        .flatten()
                        .collect(),
                );
                self.sessions.push(session);
            }
            1 => {
                let mut window = Window::from(name, self.layout());
                let children = self.panes.split_off(0);
                window.push_all(children);
                window.commands(
                    self.commands_hierarchy
                        .clone()
                        .into_iter()
                        .flatten()
                        .collect(),
                );
                self.windows.push(window);
            }
            _ => panic!("Cannot nest panes, support for this was dropped in alpha"),
        }
    }

    fn is_on_node(&self) -> bool {
        self.prev_depth <= self.depth
    }

    fn handle_event<'a, B: std::io::BufRead>(&mut self, event: Event<'a>, reader: &mut Reader<B>) {
        match event {
            Event::Start(ref e) => {
                for attr in e.attributes() {
                    let unwrapped = attr.unwrap();
                    if unwrapped.key == quick_xml::name::QName(b"layout") {
                        let layout_value = unwrapped.decode_and_unescape_value(&reader).unwrap();
                        match Layout::from(&layout_value) {
                            Ok(res) => self.layout = Some(res),
                            Err(err) => panic!(err),
                        };
                    }
                }
                self.prev_depth = self.depth;
                self.depth += 1;
                self.commands_hierarchy.resize(self.depth + 1, Vec::new());
            }
            Event::Empty(ref e) => {
                for attr in e.attributes() {
                    let unwrapped = attr.unwrap();
                    if unwrapped.key == quick_xml::name::QName(b"layout") {
                        let layout_value = unwrapped.decode_and_unescape_value(&reader).unwrap();
                        match Layout::from(&layout_value) {
                            Ok(res) => self.layout = Some(res),
                            Err(err) => panic!(err),
                        };
                    }
                }
                self.prev_depth = self.depth + 1;
                let name = String::from(str::from_utf8(e.name().into_inner()).unwrap());
                self.handle_leaf(name);
            }
            Event::Text(ref e) => {
                let decoded_command = e.unescape().unwrap();
                self.commands_hierarchy[self.depth].push(decoded_command.to_string());
            }
            Event::End(ref e) => {
                self.depth -= 1;
                let name = String::from(str::from_utf8(e.name().into_inner()).unwrap());
                if self.is_on_node() {
                    self.handle_leaf(name);
                } else {
                    self.handle_node(name);
                }
                self.commands_hierarchy.resize(self.depth + 1, Vec::new());
            }
            _ => (),
        }
    }

    pub fn parse<B: std::io::BufRead>(reader: &mut Reader<B>) -> Vec<Session> {
        let mut buf = Vec::new();
        let mut state = Parser::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Eof) => break, // break loop when event at end of file
                Ok(event) => Parser::handle_event(&mut state, event, reader),
                Err(error) => panic!(
                    "Error at position {}: {:?}",
                    reader.buffer_position(),
                    error
                ),
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

    #[test]
    fn test_empty() {
        assert_eq!(Parser::from_string(""), Vec::new());
    }

    #[test]
    fn test_singular() {
        assert_eq!(
            Parser::from_string("<base></base>"),
            vec![Session::from("base".to_string())]
        );
    }

    #[test]
    fn test_self_closing_singular() {
        assert_eq!(
            Parser::from_string("<base/>"),
            vec![Session::from("base".to_string())]
        );
    }

    #[test]
    fn test_session_can_have_commands() {
        let mut session = Session::from("session".to_string());
        session.commands(vec![String::from("test command")]);
        assert_eq!(
            Parser::from_string("<session>test command</session>"),
            vec![session]
        );
    }

    #[test]
    fn test_window_can_have_commands() {
        let mut session = Session::from("session".to_string());
        let mut window = Window::from("window".to_string(), Layout::EvenVertical);
        session.commands(vec![String::from("session command")]);
        window.commands(vec![
            String::from("session command"),
            String::from("test command"),
        ]);
        session.push_all(vec![window]);
        assert_eq!(
            Parser::from_string("<session>session command<window>test command</window></session>"),
            vec![session]
        );
    }

    #[test]
    fn test_window_self_closing_can_have_layout() {
        let mut session = Session::from(String::from("session"));
        let window = Window::from(String::from("window"), Layout::MainVertical);
        session.push_all(vec![window]);
        assert_eq!(
            Parser::from_string("<session><window layout=\"main-vertical\"/></session>"),
            vec![session]
        );
    }

    #[test]
    fn test_window_can_have_layout() {
        let mut session = Session::from(String::from("session"));
        let window = Window::from(String::from("window"), Layout::MainHorizontal);
        session.push_all(vec![window]);
        assert_eq!(
            Parser::from_string("<session><window layout=\"main-horizontal\"></window></session>"),
            vec![session]
        );
    }

    #[test]
    #[should_panic]
    fn test_panics_on_unknown_layout() {
        let mut session = Session::from(String::from("session"));
        let window = Window::from(String::from("window"), Layout::MainHorizontal);
        session.push_all(vec![window]);
        assert_eq!(
            Parser::from_string("<session><window layout=\"wrong\"></window></session>"),
            vec![session]
        );
    }

    #[test]
    fn test_self_closing_multiple_panes() {
        let mut session = Session::from(String::from("session"));
        let mut window = Window::from(String::from("window"), Layout::EvenVertical);
        let pane_one = Pane::from(String::from("pane_one"));
        let pane_two = Pane::from(String::from("pane_two"));
        window.push_all(vec![pane_one, pane_two]);
        session.push_all(vec![window]);
        let expected = vec![session];
        assert_eq!(
            Parser::from_string("<session><window><pane_one/><pane_two/></window></session>"),
            expected
        );
    }

    #[test]
    fn test_multiple_xml() {
        let expected = vec![
            Session::from("one".to_string()),
            Session::from("two".to_string()),
        ];
        assert_eq!(Parser::from_string("<one></one><two></two>"), expected);
    }

    #[test]
    #[should_panic]
    fn test_cannot_nest_panes() {
        assert_eq!(
            Parser::from_string(
                "<session><window><pane><one></one></two></two></pane></window></session>"
            ),
            Vec::new()
        );
    }
}
