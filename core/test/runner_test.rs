#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmux_version_on_path() {
        assert_eq!(version(), Ok(String::from("tmux 2.9")));
    }

}
