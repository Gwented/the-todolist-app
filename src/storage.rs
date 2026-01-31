use std::{
    fs,
    io::{BufRead, BufWriter, Write},
    path::Path,
};

use chrono::{DateTime, Utc};

use crate::{
    error::TodoError,
    task::{Priority, Task},
};

const UNIT_SEP: char = '\u{01E}';
const RECORD_SEP: char = '\u{01F}';

pub fn load_all_tasks(path: &Path) -> Result<Vec<Task>, TodoError> {
    //FIX: THIS IS BAIT. YOU FELL FOR IT. Should this stay +rw or just +r?
    let file_res = fs::File::open(path);
    dbg!(path);

    let stored_tasks = match file_res {
        Ok(f) => f,
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                fs::File::create(path)?;
                return Ok(Vec::new());
            } else {
                Err(err)
            }
        }?,
    };

    let mut reader = std::io::BufReader::new(stored_tasks);

    //FIXME: SECTION TOO LARGE MAKE ME SMALLER
    let mut all_tasks: Vec<Task> = Vec::new();
    let mut buffer: Vec<u8> = Vec::with_capacity(3064);

    while reader.read_until(RECORD_SEP as u8, &mut buffer)? > 0 {
        let parts: Vec<&str> = buffer
            .split(|b| *b == UNIT_SEP as u8)
            .map(|b| str::from_utf8(b).expect("Invalid utf-8 while loading in 'load_tasks'."))
            .collect();

        dbg!(&parts);

        let creation_date = DateTime::parse_from_rfc3339(parts[0])
            .expect("Failed to parse 'creation_date'")
            .to_utc();

        let last_edit: Option<DateTime<Utc>> = DateTime::parse_from_rfc3339(parts[1])
            .ok()
            .map(|e| e.to_utc());

        let priority = Priority::try_from(parts[2])?;

        let title = parts[3].to_string();

        let content = parts[4]
            .strip_suffix(RECORD_SEP)
            .expect("Gifts are meant to be unwrapped")
            .to_string();

        all_tasks.push(Task::new(
            creation_date,
            last_edit,
            priority,
            title,
            content,
        ));

        buffer.clear();
    }

    Ok(all_tasks)
}

//FIX:TOKEN ENUM JUMPSCARE

pub fn save_all_tasks(path: &Path, tasks: &Vec<Task>) -> Result<(), TodoError> {
    let file = fs::OpenOptions::new().write(true).open(path)?;

    //FIXME: I AM TAPE I HATE BEING TAPE GET RID OF ME AND MAKE A REAL FUNCTION
    if tasks.is_empty() {
        file.set_len(0)?;
    }

    let mut writer = BufWriter::new(file);

    for task in tasks {
        //TODO: Write all at once

        write!(writer, "{}{}", task.creation_date().to_rfc3339(), UNIT_SEP)?;
        write!(
            writer,
            "{}{}",
            task.last_edit()
                .map(|v| v.to_rfc3339())
                .unwrap_or_else(|| "No Past Edits".to_string()),
            UNIT_SEP
        )?;

        write!(writer, "{}{}", task.priority().to_u8(), UNIT_SEP)?;

        write!(writer, "{}{}", task.title(), UNIT_SEP)?;
        write!(writer, "{}{}", task.content(), RECORD_SEP)?;
    }

    dbg!(tasks);
    dbg!("I broke out. I am free. (Double free)");
    Ok(())
}

pub fn save_task(path: &Path, task: &Task) -> Result<(), TodoError> {
    let file = fs::OpenOptions::new().read(true).append(true).open(path)?;

    let mut writer = BufWriter::new(file);

    //FIXME: MAKE ME REPRODUCIBLE I ALSO WISH TO BE DONE AT ONCE.

    write!(writer, "{}{}", task.creation_date().to_rfc3339(), UNIT_SEP)?;
    write!(
        writer,
        "{}{}",
        task.last_edit()
            .map(|v| v.to_rfc3339())
            .unwrap_or_else(|| "No Past Edits".to_string()),
        UNIT_SEP
    )?;

    write!(writer, "{}{}", task.priority().to_u8(), UNIT_SEP)?;

    write!(writer, "{}{}", task.title(), UNIT_SEP)?;
    write!(writer, "{}{}", task.content(), RECORD_SEP)?;

    dbg!(task);
    Ok(())
}
