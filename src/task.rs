use std::fmt::Display;

use chrono::{DateTime, Local, Utc};

use crate::error::{ErrorContext, TodoError};

#[derive(Debug)]
pub struct Task {
    creation_date: DateTime<Utc>,
    last_edit: Option<DateTime<Utc>>,
    priority: Priority,
    title: String,
    content: String,
}

impl Task {
    pub fn new(
        creation_date: DateTime<Utc>,
        last_edit: Option<DateTime<Utc>>,
        priority: Priority,
        title: String,
        content: String,
    ) -> Self {
        Task {
            creation_date,
            last_edit,
            priority,
            title,
            content,
        }
    }

    pub fn creation_date(&self) -> &DateTime<Utc> {
        &self.creation_date
    }

    pub fn last_edit(&self) -> &Option<DateTime<Utc>> {
        &self.last_edit
    }
    pub fn priority(&self) -> &Priority {
        &self.priority
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.last_edit {
            Some(time) => {
                let time = time.with_timezone(&Local);
                write!(
                    // No, no editing check. Get out. GET OUT. FIX: Fine.
                    f,
                    "[Created: {}] [Last edit: {}]\n(Priority={}) Title: {}\nContent: \"{}\"\n",
                    self.creation_date,
                    time,
                    self.priority,
                    self.title,
                    self.content
                )
            }
            None => {
                write!(
                    // No, no editing check. Get out. GET OUT. FIX: Fine.
                    f,
                    "[Created: {}] [Last edit: Unedited]\n(Priority={}) Title: {}\nContent: \"{}\"\n",
                    self.creation_date,
                    self.priority,
                    self.title,
                    self.content
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Priority {
    pub fn to_u8(&self) -> u8 {
        match self {
            Priority::High => 0,
            Priority::Medium => 1,
            Priority::Low => 2,
        }
    }
}

impl<'a> TryFrom<&'a str> for Priority {
    type Error = TodoError<'a>;

    fn try_from(val: &'a str) -> Result<Self, Self::Error> {
        match val {
            "-t1" | "0" => Ok(Priority::High),
            "-t2" | "1" => Ok(Priority::Medium),
            "-t3" | "2" => Ok(Priority::High),
            s => {
                //FIXME: Display help
                return Err(TodoError::InvalidSyntax(ErrorContext {
                    id: Some(s),
                    help: None,
                }));
            }
        }
    }
}

impl<'a> TryFrom<&'a String> for Priority {
    type Error = TodoError<'a>;

    fn try_from(val: &'a String) -> Result<Self, Self::Error> {
        match val.as_str() {
            "-t1" => Ok(Priority::High),
            "-t2" => Ok(Priority::Medium),
            "-t3" => Ok(Priority::High),
            s => {
                return Err(TodoError::InvalidSyntax(ErrorContext {
                    id: Some(s),
                    help: None,
                }));
            }
        }
    }
}

impl From<u8> for Priority {
    fn from(val: u8) -> Self {
        match val {
            0 => Priority::Low,
            1 => Priority::Medium,
            2 => Priority::High,
            _ => unreachable!("CRITICAL ERROR: I haven't used Go yet."),
        }
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::High => write!(f, "High"),
            Priority::Medium => write!(f, "Medium"),
            Priority::Low => write!(f, "Low"),
        }
    }
}
