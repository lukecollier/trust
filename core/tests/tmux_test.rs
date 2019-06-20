use trust_core::tmux::*;

#[test]
fn tmux_version_on_path() {
    assert_eq!(version(), Ok(String::from("tmux 2.9")));
}


#[test]
fn tmux_can_create_and_kill_with_duplicates() {
    assert_eq!(create_session("target_one", "first_window"), Ok(String::from("")));
    assert_eq!(kill_session("target_one"), Ok(String::from("")));
}

#[test]
fn tmux_can_create_and_kill_with_errors() {
    assert_eq!(kill_session("target_two"), 
               Err(String::from("can\'t find session: target_two")));
}

#[test]
fn tmux_can_list_sessions() {
    assert_eq!(create_session("target_three", "first_window"), Ok(String::from("")));
    let result = list_sessions("#{session_name}:#{session_attached}").unwrap();
    assert_eq!(result.contains("target_three"), true);
    assert_eq!(kill_session("target_three"), Ok(String::from("")));
}

#[test]
fn tmux_can_make_new_window() {
    assert_eq!(create_session("target_four", "first_window"), Ok(String::from("")));
    assert_eq!(new_window("target_four", "second_window"), Ok(String::from("")));
    assert_eq!(kill_session("target_four"), Ok(String::from("")));
}

#[test]
fn tmux_can_select_layout() {
    assert_eq!(create_session("target_five", "first_window"), Ok(String::from("")));
    assert_eq!(new_window("target_five", "second_window"), Ok(String::from("")));
    assert_eq!(select_layout("target_five", "even-horizontal"), Ok(String::from("")));
    assert_eq!(kill_session("target_five"), Ok(String::from("")));
}

#[test]
fn tmux_can_split_window() {
    assert_eq!(create_session("target_six", "first_window"), Ok(String::from("")));
    assert_eq!(split_window("target_six"), Ok(String::from("")));
    assert_eq!(kill_session("target_six"), Ok(String::from("")));
}

#[test]
fn tmux_can_send_command() {
    assert_eq!(create_session("target_seven", "first_window"), Ok(String::from("")));
    assert_eq!(send_command("target_seven:first_window", "echo \"test\""), Ok(String::from("")));
    assert_eq!(kill_session("target_seven"), Ok(String::from("")));
}

#[test]
fn tmux_can_have_session() {
    assert_eq!(create_session("target_eight", "first_window"), Ok(String::from("")));
    assert_eq!(has_session("target_eight"), true);
    assert_eq!(kill_session("target_eight"), Ok(String::from("")));
}

#[test]
fn tmux_can_not_have_session() {
    assert_eq!(has_session("target_nine"), false);
}
