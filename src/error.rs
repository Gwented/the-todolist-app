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

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Branch::Main => write!(f, "(arbitrary grammar :C)"),
            Branch::NewTask => write!(f, "creating"),
            Branch::EditTask => write!(f, "editing"),
            Branch::ShowTask => write!(f, "showing"),
            Branch::SaveTask => write!(f, "saving"),
            Branch::RemoveTask => write!(f, "removing"),
        }
    }
}

#[derive(Debug)]
pub struct ErrorContext {
    pub id: Option<String>,
    pub branch: Branch,
    // pub expected: Option<String>,
    // possible format metadata
}

#[derive(Debug)]
pub enum TodoError {
    InvalidSyntax(ErrorContext),
    TitleNotFound(String),
    IO(io::ErrorKind),
}

impl From<io::Error> for TodoError {
    fn from(e: std::io::Error) -> Self {
        TodoError::IO(e.kind())
    }
}

// FIX: Enums.
const ALL_CMDS: [&'static str; 4] = ["new", "edit", "show", "rm"];

fn find_similar(id: &Option<String>) -> Option<Vec<&str>> {
    if let Some(s) = id {
        if s.len() == 1 || s.len() > 10 {
            //FIX: Show help
            //no
            return None;
        }

        let median = s.len() / 2;
        // Always favors right due to rounding so offset needed
        let (prefix, suffix) = (&s[0..median + 1], &s[median..s.len()]);
        dbg!(prefix, suffix);

        let temp: Vec<&str> = ALL_CMDS
            .iter()
            .filter(|cmd| cmd.contains(prefix) || cmd.contains(suffix))
            .map(|s| *s)
            .collect();

        return Some(temp);
    }

    None
}

//FIX: Delegate to function for specific formatting
impl Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoError::InvalidSyntax(err_ctx) => match &err_ctx.id {
                Some(id) => {
                    let help = find_similar(&err_ctx.id);
                    write!(
                        f,
                        "{ERR}: Identifier '{id}' is not valid while {} task. Help: {:#?}",
                        err_ctx.branch, help
                    )
                }
                None => {
                    let help = find_similar(&err_ctx.id).unwrap_or_default();
                    write!(f, "{ERR}: While {} task. Help: {:#?}", err_ctx.branch, help)
                }
            },
            TodoError::TitleNotFound(title) => {
                write!(f, "{ERR}: Task with title '{title}' not found.")
            }
            TodoError::IO(err) => write!(f, "{ERR}: {err}"),
        }
    }
}
