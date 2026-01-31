use std::fmt::Display;

use crate::task::Priority;

//FIXME: I AM A TAPE ENUM. I AM HERE FOR THE SOLE PURPOSE
//OF BEING IGNORED AND COVERING BOILERPLATE. I WANT STRUCTS.
#[derive(Debug)]
pub enum Options {
    Priority(Priority),
    All,
    Illegal(String),
}

impl From<&str> for Options {
    fn from(val: &str) -> Options {
        match val {
            "-p1" => Options::Priority(Priority::Low),
            "-p2" => Options::Priority(Priority::Medium),
            "-p3" => Options::Priority(Priority::High),
            "-a" => Options::All,
            val => Options::Illegal(val.to_string()),
        }
    }
}

impl From<&String> for Options {
    fn from(val: &String) -> Options {
        match val.as_str() {
            "-p1" => Options::Priority(Priority::Low),
            "-p2" => Options::Priority(Priority::Medium),
            "-p3" => Options::Priority(Priority::High),
            "-a" => Options::All,
            val => Options::Illegal(val.to_string()),
        }
    }
}

impl Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Options::Priority(p) => write!(f, "{p}"),
            Options::All => write!(f, "-a"),
            Options::Illegal(t) => write!(f, "{t}"),
        }
    }
}
