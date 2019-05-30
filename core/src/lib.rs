pub mod tmux;
pub mod parser;
pub mod runner;
mod data;

pub fn get_session_names() -> Vec<String> {
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".config");
            path.push("trust");
            path.push("sessions");
            path.set_extension("xml");
            let parsed = parser::Parser::from_file(path.as_path());
            parsed.into_iter().map(|session| session.name).collect()
        },
        None => Vec::new(),
    }
}
