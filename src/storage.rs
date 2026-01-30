use std::{
    fs,
    io::{BufRead, BufWriter, Write},
};

use chrono::{DateTime, Utc};

use crate::{
    error::TodoError,
    task::{Priority, Task},
};

const TASK_FILE: &'static str = "tasks.txt";
const UNIT_SEP: char = 0x1E as char;
const RECORD_SEP: char = 0x1F as char;

pub fn load_tasks<'a>() -> Result<Vec<Task>, TodoError<'a>> {
    let file_res = fs::File::open(TASK_FILE);

    let stored_tasks = match file_res {
        Ok(f) => f,
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                fs::File::create(TASK_FILE)?;
                return Ok(Vec::new());
            } else {
                Err(err)
            }
        }?,
    };

    let mut reader = std::io::BufReader::new(stored_tasks);

    //FIXME: SECTION TOO LARGE
    let mut all_tasks: Vec<Task> = Vec::new();
    let mut buffer: Vec<u8> = Vec::with_capacity(4128);

    while reader.read_until(RECORD_SEP as u8, &mut buffer)? > 0 {
        let parts: Vec<String> = buffer
            .split(|b| *b == UNIT_SEP as u8)
            .map(|b| String::from(String::from_utf8_lossy(b)))
            .collect();
        //FIXME: A SINGULAR. NEW LINE. 10 MINUTES DEBUGGING A NEW LINE.

        dbg!(&parts);

        let creation_date = DateTime::parse_from_rfc3339(&parts[0])
            .expect("Failed to parse 'creation_date'")
            .to_utc();

        let last_edit: Option<DateTime<Utc>> = DateTime::parse_from_rfc3339(&parts[1])
            .ok()
            .map(|e| e.to_utc());

        let priority = Priority::try_from(&parts[2])?;
        let title = parts[3].to_string();
        let content = parts[4].to_string();

        all_tasks.push(Task::new(
            creation_date,
            last_edit,
            Priority::from(priority),
            title,
            content,
        ));

        buffer.clear();
    }

    Ok(all_tasks)
}

pub fn save_tasks<'a>(tasks: &Vec<Task>) -> Result<(), TodoError<'a>> {
    let file = fs::OpenOptions::new()
        .read(true)
        .append(true)
        .open(TASK_FILE)?;

    let mut writer = BufWriter::new(file);

    for task in tasks {
        write!(writer, "{}{}", task.creation_date().to_rfc3339(), UNIT_SEP)?;
        write!(
            writer,
            "{}{}",
            task.last_edit()
                .map(|v| v.to_rfc3339())
                .unwrap_or_else(|| "No Past Edits".to_string()),
            UNIT_SEP
        )?;

        write!(writer, "{}{}", task.priority().to_u8(), UNIT_SEP,)?;

        write!(writer, "{}{}", task.title(), UNIT_SEP)?;
        write!(writer, "{}{}", task.content(), RECORD_SEP)?;
    }

    dbg!(tasks);
    Ok(())
}

// Is it a lie to not transfer ownership (Q)
pub fn edit_task(task: &mut Task) {}
