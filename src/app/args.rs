//! Args for the application.
//!
//! These args correspond to the matches in the file `clap.rs`.
//! We have these args in their own file in order to be flexible,
//! such as being able to start our app with other arg parsers.

use std::default::Default;
use crate::constants::*;

#[derive(Debug)]
pub struct Args {

    /// Test flag that sets whether the app prints diagnostics.
    /// Example: true means print diagnostics.
    pub(crate) test: bool,

    /// Log level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.
    /// Example: 5 means print debug diagnostics.
    pub(crate) log_level: Option<::log::Level>,

    // File separator string.
    // Example: "␜\n".
    pub(crate) file_separator: String,

    // Group separator string.
    // Example: "␝\n".
    pub(crate) group_separator: String,

    // Record separator string.
    // Example: "␞\n".
    pub(crate) record_separator: String,

    // Unit separator string.
    // Example: "␟".
    pub(crate) unit_separator: String,

}

impl Default for Args {
    fn default() -> Args {
        Args {
            test: false,
            log_level: None,
            file_separator: FILE_SEPARATOR_DEFAULT.into(),
            group_separator: GROUP_SEPARATOR_DEFAULT.into(),
            record_separator: RECORD_SEPARATOR_DEFAULT.into(),
            unit_separator: UNIT_SEPARATOR_DEFAULT.into(),
        }
    }
}
