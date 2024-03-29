mod data;
pub mod parser;
pub mod runner;
pub mod tmux;

// new destination for parsing and everything
pub fn get_session_names() -> Vec<String> {
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".config");
            path.push("trust");
            path.push("sessions");
            path.set_extension("xml");
            let parsed = parser::Parser::from_file(path.as_path());
            parsed.into_iter().map(|session| session.name).collect()
        }
        None => Vec::new(),
    }
}
