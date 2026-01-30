use std::{fmt::Display, io};

// Not hard-coded please keep walking
const RED: &'static str = "\x1b[31m";
const RESET: &'static str = "\x1b[0m";
const ERR: &'static str = "\x1b[31mError\x1b[0m";

#[derive(Debug)]
pub struct ErrorContext<'a> {
    pub id: Option<&'a str>,
    pub help: Option<String>,
    // possible format metadata
}

#[derive(Debug)]
pub enum TodoError<'a> {
    InvalidSyntax(ErrorContext<'a>),
    TitleNotFound(&'a str),
    TaskNotFound(&'a str),
    IO(io::ErrorKind),
}

impl From<io::Error> for TodoError<'_> {
    fn from(e: std::io::Error) -> Self {
        TodoError::IO(e.kind())
    }
}

impl std::error::Error for TodoError<'_> {}

impl Display for TodoError<'_> {
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
