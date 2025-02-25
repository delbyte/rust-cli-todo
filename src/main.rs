use std::fs::{OpenOptions, File};
use std::io::{Write, BufRead, BufReader};
use std::env;

const FILE_PATH: &str = "data.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    let iter = &args[1..];

    if iter.is_empty() {
        eprintln!("Usage: <Command> <Task>");
        return;
    }

    match iter[0].to_lowercase().as_str() {
        "add" => add_task(&iter[1..]),
        "list" => list_tasks(),
        "complete" => complete_task(&iter[1..]),
        "remove" => remove_task(&iter[1..]),
        _ => eprintln!("Invalid command. Use 'add', 'list', 'complete', or 'remove'."),
    }
}

// Function to read all tasks from file
fn read_tasks() -> Vec<String> {
    let file = File::open(FILE_PATH).unwrap_or_else(|_| File::create(FILE_PATH).expect("Failed to create file"));
    BufReader::new(file).lines().filter_map(Result::ok).collect()
}

// Function to write tasks back to file (after modification)
fn write_tasks(tasks: &[String]) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .expect("Failed to open file for writing");

    for task in tasks {
        writeln!(file, "{}", task).expect("Failed to write to file.");
    }
}

// Add Task
fn add_task(task_parts: &[String]) {
    if task_parts.is_empty() {
        eprintln!("Usage: add <Task>");
        return;
    }

    let task_string = format!("[pending] {}", task_parts.join(" "));

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(FILE_PATH)
        .expect("Failed to open file.");

    writeln!(file, "{}", task_string).expect("Failed to write to file.");

    println!("Task added!");
    list_tasks();
}

// List Tasks
fn list_tasks() {
    let tasks = read_tasks();
    println!("\nCurrent tasks:");
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for (num, task) in tasks.iter().enumerate() {
            println!("{}. {}", num + 1, task);
        }
    }
}

// Complete Task
fn complete_task(task_parts: &[String]) {
    if task_parts.is_empty() {
        eprintln!("Usage: complete <Task Index>");
        return;
    }

    let mut tasks = read_tasks();
    let task_index: usize = match task_parts[0].parse() {
        Ok(index) => index,
        Err(_) => {
            eprintln!("Invalid task index.");
            return;
        }
    };

    if task_index == 0 || task_index > tasks.len() {
        eprintln!("Task index out of range.");
        return;
    }

    tasks[task_index - 1] = format!("[complete] {}", tasks[task_index - 1].trim_start_matches("[pending] "));
    write_tasks(&tasks);
    println!("Task marked as complete!");
    list_tasks();
}

// Remove Task
fn remove_task(task_parts: &[String]) {
    if task_parts.is_empty() {
        eprintln!("Usage: remove <Task Index>");
        return;
    }

    let mut tasks = read_tasks();
    let task_index: usize = match task_parts[0].parse() {
        Ok(index) => index,
        Err(_) => {
            eprintln!("Invalid task index.");
            return;
        }
    };

    if task_index == 0 || task_index > tasks.len() {
        eprintln!("Task index out of range.");
        return;
    }

    println!("Removed task: {}", tasks[task_index - 1]);
    tasks.remove(task_index - 1);
    write_tasks(&tasks);
    list_tasks();
}