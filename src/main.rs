use std::fs::OpenOptions;
use std::{fs, io};

use anyhow::{anyhow, Result};
use chrono::{Duration, Utc};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};

enum ParsedInput {
    ValidNumber(u64),
    InvalidInput(String),
}

fn is_string_numeric(str: &str) -> bool {
    !str.chars().any(|c| !c.is_numeric())
}

struct DurationConverter;

impl DurationConverter {
    fn to_seconds(value: &str) -> ParsedInput {
        if is_string_numeric(value) {
            match value.parse::<u64>() {
                Ok(n) => ParsedInput::ValidNumber(n),
                Err(_) => return ParsedInput::InvalidInput(value.to_string()),
            }
        } else {
            let multiplier = match value.chars().last().unwrap_or_else(|| 's') {
                's' => 1,
                'm' => 60,
                'h' => 3600,
                'd' => 86400,
                'w' => 604800,
                'M' => 2592000,
                'y' => 31536000,
                _ => 0,
            };
            let number = value
                .strip_suffix(|_: char| true)
                .unwrap_or_else(|| "1")
                .parse::<u64>();

            match number {
                Ok(n) => ParsedInput::ValidNumber(n * multiplier),
                Err(_) => ParsedInput::InvalidInput(value.to_string()),
            }
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    title: String,
    description: String,
    due: chrono::DateTime<chrono::Utc>,
}

fn create_task() -> Result<Task> {
    let mut task_name = String::new();
    let mut task_desc = String::new();
    let mut task_due = String::new();

    println!("-------------------------------");
    println!("Enter the Task's Name: ");
    io::stdin().read_line(&mut task_name)?;
    println!("Enter the Task's Description: ");
    io::stdin().read_line(&mut task_desc)?;
    println!("When is it due?");
    println!("GUIDE:[s:Seconds, m:Minutes, h:Hours, d:Days, w:Weeks, M:Months, y:Years]");
    println!("Example: 30m :: 30 Minutes");
    io::stdin().read_line(&mut task_due)?;
    println!("-------------------------------");

    match DurationConverter::to_seconds(&task_due.trim().to_string()) {
        ParsedInput::ValidNumber(number) => Ok(Task {
            title: task_name.trim().to_string(),
            description: task_desc.trim().to_string(),
            due: chrono::Utc::now() + Duration::seconds(number as i64),
        }),
        ParsedInput::InvalidInput(failed_number) => {
            println!("{:?} is not a valid positive number.", failed_number);
            Err(anyhow!(
                "Failed to create task, cannot parse input of {:?}",
                failed_number
            ))
        }
    }
}

fn main() -> anyhow::Result<()> {
    const PATH: &str = "tasks.ron";
    // Ensure Creation
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&PATH)?;

    let mut data = fs::read_to_string(&PATH)?;

    let mut tasks = ron::from_str::<Vec<Task>>(&data).unwrap_or_else(|_| Vec::new());

    loop {
        let mut choice = String::new();

        println!("Want to create another task?");
        io::stdin().read_line(&mut choice)?;
        match choice.trim().to_lowercase().as_str() {
            "y" => {
                match create_task() {
                    Ok(task) => tasks.push(task),
                    Err(ex) => {
                        println!("{}", ex);
                        continue;
                    }
                }
                continue;
            }
            _ => break,
        }
    }

    data = to_string_pretty(&tasks, PrettyConfig::new())?;
    fs::write(&PATH, &data)?;

    let mut tasks = ron::from_str::<Vec<Task>>(&data)?;

    for task in tasks.iter() {
        println!("----------------------");
        println!("{:?}", task);
        println!("......................");
        match task.due <= Utc::now() {
            true => {
                println!("It is time to complete this task.");
            }
            false => println!("It is not time to complete this task yet."),
        }
    }

    tasks.retain(|t| t.due >= Utc::now());

    data = to_string_pretty(&tasks, PrettyConfig::new())?;
    fs::write(&PATH, &data)?;

    Ok(())
}
