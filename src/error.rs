//FIX: WHY. Ok that's it.
use std::{fmt::Display, io};

// Not hard-coded please keep walking
// const RED: &'static str = "\x1b[31m";
// const RESET: &'static str = "\x1b[0m";
const ERR: &'static str = "\x1b[31mError\x1b[0m";

#[derive(Debug, PartialEq, Eq)]
pub enum Branch {
    Main,
    NewTask,
    EditTask,
    ShowTask,
    SaveTask,
    RemoveTask,
}

#[derive(Debug)]
pub struct ErrorContext {
    pub id: Option<String>,
    pub branch: Branch,
    // pub expected: Option<String>,
    pub help: Option<String>,
    // possible format metadata
}

//FIX:TOKEN ENUM JUMPSCARE

#[derive(Debug)]
pub enum TodoError {
    InvalidSyntax(ErrorContext),
    TitleNotFound(String),
    TaskNotFound(String),
    IO(io::ErrorKind),
}

impl From<io::Error> for TodoError {
    fn from(e: std::io::Error) -> Self {
        TodoError::IO(e.kind())
    }
}

impl Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoError::InvalidSyntax(err_ctx) => {
                write!(f, "{{TEMP}}Error with identifier {err_ctx:?}")
            }
            TodoError::TitleNotFound(title) => {
                write!(f, "{ERR}: Task with title '{title}' not found.")
            }
            TodoError::TaskNotFound(task) => {
                write!(f, "{ERR}: Task with title '{task}' not found.")
            }
            TodoError::IO(err) => write!(f, "{ERR}: {err}"),
        }
    }
}
