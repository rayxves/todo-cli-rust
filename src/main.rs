mod todo_list;

use clap::Parser;
use colored::*;
use todo_list::{complete_task, create_new_task, remove_task, update_task_completion_time, update_task_name, view_concluded_tasks, view_tasks, Args};

fn main() {
    let args = Args::parse();

    match &args {
        Args { add_task: Some(_), .. } => create_new_task(&args).unwrap(),
        Args { remove_task: Some(_), .. } => remove_task(&args).unwrap(),
        Args { update_name: Some(_), .. } => update_task_name(&args).unwrap(),
        Args { update_concluded_time: Some(_), .. } => update_task_completion_time(&args).unwrap(),
        Args { mark_concluded_task: Some(_), .. } => complete_task(&args).unwrap(),
        Args { view_tasks: Some(true), .. } => view_tasks(&args).unwrap(),
        Args { view_concluded_tasks: Some(true), .. } => view_concluded_tasks(&args).unwrap(),
        _ => println!("{}", "No valid command provided.".red())
    }
}
