use anyhow::Result;
use std::{
    io,
    sync::{Arc, Mutex},
    thread,
};

fn read_user_input(tasks: Arc<Mutex<Vec<Task>>>) {
    loop {
        let mut check = String::new();
        println!("Want to add a task? [Y/N]: ");
        io::stdin().read_line(&mut check).unwrap(); // for now

        if check.to_lowercase().trim() == "y" {
            let mut task_name = String::new();
            let mut task_desc = String::new();

            println!("-------------------------------");
            println!("Enter the Task's Name: ");
            io::stdin().read_line(&mut task_name).unwrap(); // for now
            println!("Enter the Task's Description: ");
            io::stdin().read_line(&mut task_desc).unwrap(); // for now
            println!("-------------------------------");

            let new_task = Task::new(task_name, task_desc, 10);
            tasks.lock().unwrap().append(&mut vec![new_task])
        } else {
            break;
        }
    }
}

fn check_tasks(tasks: Arc<Mutex<Vec<Task>>>) {
    for task in tasks.lock().unwrap().iter() {
        if task.is_finished {
            println!("Task \"{:?}\" was completed.", task.name);
        }
    }
}

fn main() -> Result<()> {
    let tasks = Arc::new(Mutex::new(vec![]));

    let input_tasks_pointer = tasks.clone();
    let check_tasks_pointer = tasks.clone();

    let check_tasks_handle = thread::spawn(|| check_tasks(check_tasks_pointer));
    let read_user_input_handle = thread::spawn(|| read_user_input(input_tasks_pointer));
    read_user_input_handle.join();
    check_tasks_handle.join();
    Ok(())
}

struct Task {
    name: String,
    description: String,
    is_finished: bool,
    start_time: chrono::DateTime<chrono::Utc>,
    end_time: chrono::DateTime<chrono::Utc>,
}

impl Task {
    /// Creates a new Task.
    fn new<IntoStr: Into<String>>(name: IntoStr, description: IntoStr, timeout: i64) -> Self {
        let start_time = chrono::Utc::now();
        Self {
            name: name.into(),
            description: description.into(),
            is_finished: false,
            start_time: start_time,
            end_time: start_time + chrono::Duration::seconds(timeout),
        }
    }
    /// Starts the task.
    fn start(&mut self) {
        loop {
            self.update();
            if self.is_finished {
                break;
            }
        }
    }

    /// Updates the task for every duration
    fn update(&mut self) {
        if self.is_finished {
            // Finished Behavior
        }
        if self.end_time <= chrono::Utc::now() {
            self.is_finished = true;
        }
    }
}
