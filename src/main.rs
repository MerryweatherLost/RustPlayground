use std::fs::OpenOptions;
use std::{fs, io};

use chrono::Duration;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};

fn is_string_numeric(str: &String) -> bool {
    str.chars().any(|c| !c.is_numeric())
}
struct DurationConverter {
    value: String,
}
impl DurationConverter {
    fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
    fn to_seconds(&self) -> i64 {
        if is_string_numeric(&self.value.clone()) {
            match self.value.parse::<i64>() {
                Ok(n) => n,
                Err(_) => 1,
            }
        } else {
            let multiplier = match self.value.chars().last().unwrap_or_else(|| 's') {
                's' => 1,
                'm' => 60,
                'h' => 3600,
                'd' => 86400,
                'w' => 604800,
                'M' => 2592000,
                'y' => 31536000,
                _ => 0,
            };
            let number = self
                .value
                .strip_suffix(|_: char| true)
                .unwrap_or_else(|| "1")
                .parse::<i64>()
                .unwrap_or_else(|_| 1);
            number * multiplier
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    title: String,
    description: String,
    due: chrono::DateTime<chrono::Utc>,
}

fn create_task() -> Task {
    let mut task_name = String::new();
    let mut task_desc = String::new();
    let mut task_due = String::new();

    println!("-------------------------------");
    println!("Enter the Task's Name: ");
    io::stdin().read_line(&mut task_name).unwrap();
    println!("Enter the Task's Description: ");
    io::stdin().read_line(&mut task_desc).unwrap();
    println!("When is it due?");
    println!("GUIDE:[s:Seconds, m:Minutes, h:Hours, d:Days, w:Weeks, M:Months, y:Years]");
    println!("Example: 30m :: 30 Minutes");
    io::stdin().read_line(&mut task_due).unwrap();
    println!("-------------------------------");

    Task {
        title: task_name.trim().to_string(),
        description: task_desc.trim().to_string(),
        due: chrono::Utc::now()
            + Duration::seconds(DurationConverter::new(&task_due.trim().to_string()).to_seconds()),
    }
}

fn main() -> anyhow::Result<()> {
    // defines
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
                tasks.push(create_task());
                continue;
            }
            _ => break,
        }
    }

    data = to_string_pretty(&tasks, PrettyConfig::new())?;
    fs::write(&PATH, &data)?;
    let current_tasks = ron::from_str::<Vec<Task>>(&data)?;

    for task in current_tasks.iter() {
        println!("----------------------");
        println!("{:?}", task);
        println!("......................");
        match task.due <= chrono::Utc::now() {
            true => {
                println!("It is time to complete this task.");
                // current_tasks.retain(|&t| t.title == task.title); -> Cannot move out of a shared reference
            }
            false => println!("It is not time to complete this task yet."),
        }
        println!("----------------------");
    }

    Ok(())
}
