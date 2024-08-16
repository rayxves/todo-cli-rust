use clap::Parser;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::io::{self, Read, Write};
use colored::*;

/// Struct para armazenar os argumentos do CLI.
#[derive(Debug, Parser)]
#[command(author = "Rayssa", version = "1.0", about = "Todo-list created from CLI.")]
pub struct Args {
    #[arg(short, long, help = "Adds a new task; the first argument should be the task name and the second should be the completion date.", num_args(1..))]
    pub add_task: Option<Vec<String>>,

    #[arg(short, long, help = "Removes a task specified by name.")]
    pub remove_task: Option<String>,

    #[arg(short, long, help = "Changes the name of a task; the first argument is the old name and the second is the new name.", num_args(1..))]
    pub update_name: Option<Vec<String>>,

    #[arg(short = 't', long, help = "Changes the completion time of a task; the first argument is the task name and the second is the new completion time.", num_args(1..))]
    pub update_concluded_time: Option<Vec<String>>,

    #[arg(short, long, help = "Prints all incomplete tasks.")]
    pub view_tasks: Option<bool>,

    #[arg(short, long, help = "Marks a task as completed by name.")]
    pub concluded_task: Option<String>,

    #[arg(short = 'w', long, help = "Prints all completed tasks.")]
    pub view_concluded_tasks: Option<bool>
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct NewTask {
    name: String,
    completion_time: String,
}

fn read_tasks_from_file(filename: &str) -> io::Result<Vec<NewTask>> {
    let file_content = fs::read_to_string(filename).unwrap_or_else(|_| {
        fs::File::create(filename).ok();
        String::new()
    });
    Ok(serde_json::from_str(&file_content).unwrap_or_default())
}

fn write_tasks_to_file(filename: &str, tasks: &[NewTask]) -> io::Result<()> {
    let data_serialized = serde_json::to_string_pretty(tasks)?;
    fs::write(filename, data_serialized)?;
    Ok(())
}

pub fn create_new_task(args: &Args) -> io::Result<()> {
    if let Some(task) = &args.add_task {
        if task.len() == 2 {
            let new_task = NewTask {
                name: task[0].to_owned(),
                completion_time: task[1].to_owned(),
            };

            let mut tasks = read_tasks_from_file("src/serde.json")?;
            tasks.push(new_task);
            write_tasks_to_file("src/serde.json", &tasks)?;

            println!("{}", "Task added successfully.".green());
        } else {
            println!("{}", "Two arguments are required.".red());
        }
    }
    Ok(())
}

pub fn view_tasks(args: &Args) -> io::Result<()> {
    if let Some(true) = args.view_tasks {
        let tasks = read_tasks_from_file("src/serde.json")?;
        println!("{}", "\n--------------------------\nTasks to be completed: \n--------------------------".bright_red());
        for (i, task) in tasks.iter().enumerate() {
            println!("{}", format!("#{}º", i + 1).white());
            println!("{}", format!("Name: {}", task.name).green());
            println!("{}", format!("Completion deadline: {}\n", task.completion_time).green());
        }
    }
    Ok(())
}

pub fn remove_task(args: &Args) -> io::Result<()> {
    if let Some(task_to_remove) = &args.remove_task {
        let mut tasks = read_tasks_from_file("src/serde.json")?;
        tasks.retain(|task| task.name != *task_to_remove);
        write_tasks_to_file("src/serde.json", &tasks)?;
        println!("{}", "Task removed successfully.".green());
    }
    Ok(())
}

pub fn update_task_name(args: &Args) -> io::Result<()> {
    if let Some(task_to_update) = &args.update_name {
        if task_to_update.len() == 2 {
            let mut tasks = read_tasks_from_file("src/serde.json")?;
            let mut task_found = false;

            for task in tasks.iter_mut() {
                if task.name == task_to_update[0] {
                    task.name = task_to_update[1].to_owned();
                    task_found = true;
                    break;
                }
            }
            if task_found {
                write_tasks_to_file("src/serde.json", &tasks)?;
                println!("{}", "Task updated successfully.".green());
            } else {
                println!("{}", "Task not found.".red());
            }
        } else {
            println!("{}", "You must provide the current name and the new name of the task.".red());
        }
    }
    Ok(())
}

pub fn update_task_completion_time(args: &Args) -> io::Result<()> {
    if let Some(task_to_update) = &args.update_concluded_time {
        if task_to_update.len() == 2 {
            let mut tasks = read_tasks_from_file("src/serde.json")?;
            let mut task_found = false;

            for task in tasks.iter_mut() {
                if task.name == task_to_update[0] {
                    task.completion_time = task_to_update[1].to_owned();
                    task_found = true;
                    break;
                }
            }
            if task_found {
                write_tasks_to_file("src/serde.json", &tasks)?;
                println!("{}", "Task updated successfully.".green());
            } else {
                println!("{}", "Task not found.".red());
            }
        } else {
            println!("{}", "You must provide the task name and the new completion time.".red());
        }
    }
    Ok(())
}

pub fn complete_task(args: &Args) -> io::Result<()> {
    if let Some(concluded_task) = &args.concluded_task {
        let mut tasks = read_tasks_from_file("src/serde.json")?;
        let mut completed_tasks = read_tasks_from_file("src/completed_tasks.json").unwrap_or_default();
        let initial_len = tasks.len();

        tasks.retain(|task| {
            if task.name == *concluded_task {
                completed_tasks.push(task.clone());
                false
            } else {
                true
            }
        });

        if tasks.len() < initial_len {
            write_tasks_to_file("src/serde.json", &tasks)?;
            write_tasks_to_file("src/completed_tasks.json", &completed_tasks)?;
            println!("{}", "Task added to completed tasks.".green());
        } else {
            println!("{}", "Task not found.".red());
        }
    }
    Ok(())
}

pub fn view_concluded_tasks(args: &Args) -> io::Result<()> {
    if let Some(true) = args.view_concluded_tasks {
        let tasks = read_tasks_from_file("src/completed_tasks.json")?;
        println!("{}", "\n--------------------------\nCompleted tasks: \n--------------------------".bright_red());
        for (i, task) in tasks.iter().enumerate() {
            println!("{}", format!("#{}º", i + 1).white());
            println!("{}", format!("Name: {}", task.name).green());
            println!("{}", format!("Completion deadline: {}\n", task.completion_time).green());
        }
    }
    Ok(())
}
