use another_taskbar::app::parse_mode;

#[test]
fn defaults_to_gui_mode() {
    let args = vec!["another_taskbar".to_string()];
    assert_eq!(parse_mode(&args), "gui");
}

#[test]
fn parses_cli_mode_flag() {
    let args = vec!["another_taskbar".to_string(), "--cli".to_string()];
    assert_eq!(parse_mode(&args), "cli");
}

#[test]
fn parses_gui_short_flag() {
    let args = vec!["another_taskbar".to_string(), "-g".to_string()];
    assert_eq!(parse_mode(&args), "gui");
}
