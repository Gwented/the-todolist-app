use chrono::Utc;

use crate::{
    error::{ErrorContext, TodoError},
    storage,
    task::{Priority, Task},
};

// Learn from trait

pub fn exec(args: &Vec<String>) -> Result<(), TodoError<'_>> {
    match &args.get(0).map(|s| s.as_str()) {
        Some("new") => new_task(&args[1..]), // [title] ""[content] -p[tier]
        Some("edit") => new_task(&args[1..]), // [title] -t[title] or -c [content] ""[content]
        Some("show") => new_task(&args[1..]), // -a[default] -t[tier]
        Some("rm") => new_task(&args[1..]),  // [title] or -a[all]
        Some("undo") => new_task(&args[1..]),
        Some(s) => Err(TodoError::InvalidSyntax(ErrorContext {
            id: Some(s),
            help: None,
        })),
        None => Err(TodoError::InvalidSyntax(ErrorContext {
            id: None,
            help: None,
        })),
    }
}

fn new_task(args: &[String]) -> Result<(), TodoError<'_>> {
    // point of synt is hard coded logic anyways so is it that bad indexing? (Q)
    let creation_date = Utc::now();
    let mut priority = Priority::Medium;

    let title = args.get(0).ok_or(TodoError::InvalidSyntax(ErrorContext {
        id: None,
        help: None,
    }))?;

    let mut content = String::new();
    let has_content = false;

    for arg in args {
        dbg!(&arg);
        match arg {
            arg if arg.starts_with("-") || arg.starts_with("--") => {
                priority = Priority::try_from(arg)?;
            }
            // arg if arg.starts_with("--") => {}
            arg => {
                if has_content == true {
                    Err(TodoError::InvalidSyntax(ErrorContext {
                        id: Some(arg),
                        help: None,
                    }))?;
                }
                content = arg.to_string();
            }
        }
    }

    dbg!(&content, &creation_date);

    let new_task = Task::new(creation_date, None, priority, title.to_string(), content);
    dbg!(&new_task);

    let mut tasks = storage::load_tasks()?;
    tasks.push(new_task);

    storage::save_tasks(&tasks)?;

    Ok(())
}
