use chrono::Utc;

use crate::{
    args::Options,
    error::{Branch, ErrorContext, TodoError},
    iyo::{config::GlobalConfig, storage},
    task::{Priority, Task},
};

pub fn exec(global_cfg: &GlobalConfig, args: &Vec<String>) -> Result<(), TodoError> {
    match &args.get(0).map(|s| s.as_str()) {
        Some("new") | Some("n") => new_task(global_cfg, &args[1..]), // [title] ""[content] -p[tier]
        Some("edit") | Some("e") => edit_task(global_cfg, &args[1..]), // [title] -t[title] or -c [content] ""[content]
        Some("show") | Some("s") => show_task(global_cfg, &args[1..]), // -a[default] -p[tier]
        Some("rm") => remove_task(global_cfg, &args[1..]),             // [title] or -a[all]
        Some("forget") => remove_root(global_cfg),
        // Some("undo") => new_task(&args[1..]), // Why is this here?
        Some(s) => Err(TodoError::InvalidSyntax(ErrorContext {
            id: Some(s.to_string()),
            branch: Branch::Main,
        })),
        None => Err(TodoError::InvalidSyntax(ErrorContext {
            id: None,
            branch: Branch::Main,
        })),
    }
}

//FIX: REMOVE BOOLEANS
//no
fn new_task(global_cfg: &GlobalConfig, args: &[String]) -> Result<(), TodoError> {
    let creation_date = Utc::now();
    let mut priority = Priority::Medium;

    let title = args.get(0).ok_or(TodoError::InvalidSyntax(ErrorContext {
        id: None,
        branch: Branch::NewTask,
    }))?;

    let mut content = String::new();
    let has_content = false;

    for arg in args.iter().skip(1) {
        // dbg!(&arg);
        match arg {
            arg if arg.starts_with("-") || arg.starts_with("--") => match Options::from(arg) {
                Options::Priority(p) => {
                    priority = p;
                }
                t => Err(TodoError::InvalidSyntax(ErrorContext {
                    id: Some(t.to_string()),
                    branch: Branch::NewTask,
                }))?,
            },
            // arg if arg.starts_with("--") => {}
            arg => {
                if has_content == true {
                    println!("Has content twice?");
                    Err(TodoError::InvalidSyntax(ErrorContext {
                        id: Some(arg.to_string()),
                        branch: Branch::NewTask,
                    }))?;
                }
                content = arg.to_string();
            }
        }
    }

    // dbg!(&content, &creation_date);
    storage::init_dir(&global_cfg.file_path)?;

    let new_task = Task::new(creation_date, None, priority, title.to_string(), content);
    // dbg!(&new_task);

    storage::save_task(&global_cfg.file_path, &new_task)?;
    println!("\x1b[32mSaved task.\x1b[0m");

    Ok(())
}

//FIX: Green Greem Green Green Green
// const GREEN: &'static str = "\x1b[32m \x1b[0m";

fn edit_task(global_cfg: &GlobalConfig, args: &[String]) -> Result<(), TodoError> {
    let target = args.get(0).ok_or(TodoError::InvalidSyntax(ErrorContext {
        id: None,
        branch: Branch::EditTask,
    }))?;

    let mut all_tasks = storage::load_all_tasks(&global_cfg.file_path)?;

    //FIX: "use use"
    let target = all_tasks
        .iter_mut()
        .position(|t| t.title() == target)
        .ok_or(TodoError::InvalidSyntax(ErrorContext {
            id: None,
            branch: Branch::EditTask,
        }))?;

    let task = &mut all_tasks[target];

    let edit = args.get(1).ok_or(TodoError::InvalidSyntax(ErrorContext {
        id: None,
        branch: Branch::EditTask,
    }))?;

    task.set_title(edit.to_string());

    storage::save_all_tasks(&global_cfg.file_path, &all_tasks)?;

    Ok(())
}

fn show_task(global_cfg: &GlobalConfig, args: &[String]) -> Result<(), TodoError> {
    let mut all = false;

    let mut has_title = false;
    let mut target: Option<&str> = None;

    for arg in args.iter() {
        // dbg!(&arg);
        match arg {
            arg if arg.starts_with("-") || arg.starts_with("--") => match Options::from(arg) {
                Options::All => all = true,
                //  FIXME: NOW I WANT UNIFIED ERROR HANDLING
                t => Err(TodoError::InvalidSyntax(ErrorContext {
                    id: Some(t.to_string()),
                    branch: Branch::ShowTask,
                }))?,
            },
            // arg if arg.starts_with("--") => {}
            arg => {
                if has_title == true {
                    Err(TodoError::InvalidSyntax(ErrorContext {
                        id: Some(arg.to_string()),
                        branch: Branch::ShowTask,
                    }))?;
                }
                target = Some(arg);
                has_title = true;
            }
        }
    }

    let all_tasks = storage::load_all_tasks(&global_cfg.file_path)?;
    // dbg!("loaded", all);

    if all {
        for task in &all_tasks {
            println!("{task}");
        }

        return Ok(());
    }

    let target = target.ok_or_else(|| {
        TodoError::InvalidSyntax(ErrorContext {
            id: None,
            branch: Branch::ShowTask,
        })
    })?;

    // if all_tasks.is_empty() {
    //     println!("You have no tasks. You're \x1b[31mnot\x1b[0m gonna make it.");
    //     return Ok(());
    // }

    //FIXME: Ok the booleans are gettting old just create the Config structs.
    //no
    let index = all_tasks
        .iter()
        .position(|t| t.title() == target)
        .ok_or_else(|| TodoError::TitleNotFound(target.to_string()))?;

    println!("{}", all_tasks.get(index).expect("Infallible"));

    Ok(())
}

fn remove_task(global_cfg: &GlobalConfig, args: &[String]) -> Result<(), TodoError> {
    let mut has_title = false;
    let mut target: Option<&str> = None;

    let mut all = false;

    for arg in args.iter() {
        // dbg!(&arg);
        match arg {
            arg if arg.starts_with("-") || arg.starts_with("--") || arg.starts_with(".") => {
                match Options::from(arg) {
                    Options::All => all = true,
                    t => Err(TodoError::InvalidSyntax(ErrorContext {
                        id: Some(t.to_string()),
                        branch: Branch::RemoveTask,
                    }))?,
                }
            }
            // arg if arg.starts_with("--") => {}
            arg => {
                if has_title == true {
                    Err(TodoError::InvalidSyntax(ErrorContext {
                        id: Some(arg.to_string()),
                        branch: Branch::RemoveTask,
                    }))?;
                }
                target = Some(arg);
                has_title = true;
            }
        }
    }

    let mut all_tasks = storage::load_all_tasks(&global_cfg.file_path)?;

    if all {
        all_tasks.clear();
        storage::save_all_tasks(&global_cfg.file_path, &all_tasks)?;

        println!("\x1b[31mAll tasks removed.\x1b[0m");
        return Ok(());
    }

    let target = target.ok_or_else(|| {
        TodoError::InvalidSyntax(ErrorContext {
            id: None,
            branch: Branch::RemoveTask,
        })
    })?;

    //FIX: IM SICK OF BEING AN INDEX I WANT HASHING NOW

    match all_tasks.iter_mut().position(|t| t.title() == target) {
        Some(t) => all_tasks.remove(t),
        None => {
            return Err(TodoError::TitleNotFound(target.to_string()));
        }
    };

    storage::save_all_tasks(&global_cfg.file_path, &all_tasks)?;

    println!("\x1b[31mRemoved task '{target}'.\x1b[0m");

    Ok(())
}

// JAVA
fn remove_root(global_cfg: &GlobalConfig) -> Result<(), TodoError> {
    storage::remove_main(&global_cfg.file_path)
}
