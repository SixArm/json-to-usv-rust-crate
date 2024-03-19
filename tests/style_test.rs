#[path = "testing.rs"]
mod testing;
use testing::*;

#[path = "command_io.rs"]
mod command_io;
use command_io::*;

use std::process::Command;

pub const JSON_EXAMPLE: &str = r#"[[[["a","b"],["c","d"]],[["e","f"],["g","h"]]],[[["i","j"],["k","l"]],[["m","n"],["o","p"]]]]"#;

#[test]
fn command_with_style_braces() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-braces"), JSON_EXAMPLE);
    assert_eq!(actual, usv::constants::STYLE_BRACES_EXAMPLE);
}

#[test]
fn command_with_style_controls() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-controls"), JSON_EXAMPLE);
    assert_eq!(actual, usv::constants::STYLE_CONTROLS_EXAMPLE);
}

#[test]
fn command_with_style_symbols() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-symbols"), JSON_EXAMPLE);
    assert_eq!(actual, usv::constants::STYLE_SYMBOLS_EXAMPLE);
}

#[test]
fn command_with_style_liners() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-liners"), JSON_EXAMPLE);
    assert_eq!(actual, usv::constants::STYLE_LINERS_EXAMPLE);
}

#[test]
fn command_with_style_sheets() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-sheets"), JSON_EXAMPLE);
    assert_eq!(actual, usv::constants::STYLE_SHEETS_EXAMPLE);
}
