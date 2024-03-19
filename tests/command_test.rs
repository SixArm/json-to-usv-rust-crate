#[path = "testing.rs"]
mod testing;
use testing::*;

#[path = "command_io.rs"]
mod command_io;
use command_io::*;

use std::process::Command;

pub const JSON_EXAMPLE: &str = r#"[[[["a","b"],["c","d"]],[["e","f"],["g","h"]]],[[["i","j"],["k","l"]],[["m","n"],["o","p"]]]]"#;

#[test]
fn command() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(&mut command, JSON_EXAMPLE);
    assert_eq!(actual, usv::constants::STYLE_SYMBOLS_EXAMPLE);
}
