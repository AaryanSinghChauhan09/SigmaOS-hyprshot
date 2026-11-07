//! Options model.

#[derive(Debug, Clone, PartialEq)]
pub enum Mode { Default, Output(String), Format(String), Region(i32, i32, u32, u32), Window(String), Freeze }

#[derive(Debug, Clone)]
pub struct Options { pub mode: Mode, pub output: Option<String> }
impl Default for Options { fn default() -> Self { Self { mode: Mode::Default, output: None } } }
#[cfg(test)] mod tests { use super::*; #[test] fn o() { assert!(matches!(Options::default().mode, Mode::Default)); } }
