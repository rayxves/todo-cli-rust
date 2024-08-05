use clap::Parser;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::io::{self, Read, Write};
use colored::*;

#[derive(Debug, Parser)]
#[command(author = "Rayssa", version = "1.0", about = "Todo-list feito a partir de CLI.")]
pub struct Args {
    #[arg(short, long, help = "Adiciona uma nova task, o primeiro argumento deve ser o nome da task e o segundo deve ser a data de conclusão.", num_args(1..))]
    pub add_task: Option<Vec<String>>,

    #[arg(short, long, help = "Remove a task especificada pelo nome.")]
    pub remove_task: Option<String>,

    #[arg(short, long, help = "Muda o nome da task, o primeiro argumento é o antigo nome da task e o segundo é o novo nome", num_args(1..))]
    pub update_name: Option<Vec<String>>,

    #[arg(short = 't', long, help = "Muda o tempo de conclusão da task, o primeiro argumento é o nome da task e o segundo é o novo tempo para conclusão.", num_args(1..))]
    pub update_concluded_time: Option<Vec<String>>,

    #[arg(short, long, help = "Imprime todas as tasks não concluídas")]
    pub view_tasks: Option<bool>,

    #[arg(short, long, help = "Marca uma task como concluída pelo nome.")]
    pub concluded_task: Option<String>,

    #[arg(short = 'w', long, help = "Imprime todas as tasks já concluídas")]
    pub view_concluded_tasks: Option<bool>
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct NewTask {
    name: String,
    completion_time: String,
}

fn read_tasks_from_file(filename: &str) -> io::Result<Vec<NewTask>> {
    match fs::File::open(filename) {
        Ok(mut json_file) => {
            let mut data = String::new();
            json_file.read_to_string(&mut data)?;
            let tasks: Vec<NewTask> = serde_json::from_str(&data).unwrap_or_else(|_| vec![]);
            Ok(tasks)
        }
        Err(_) => {
            // Cria o arquivo se não existir
            fs::File::create(filename)?;
            Ok(vec![])
        }
    }
}

fn write_tasks_to_file(filename: &str, tasks: &[NewTask]) -> io::Result<()> {
    let data_serialized = serde_json::to_string_pretty(tasks)?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)?;
    file.write_all(data_serialized.as_bytes())?;
    Ok(())
}

pub fn create_new_task(args: &Args) -> io::Result<()> {
    if let Some(task) = &args.add_task {
        if task.len() > 1 {
            let new_task = NewTask {
                name: task[0].to_owned(),
                completion_time: task[1].to_owned(),
            };

            let mut tasks = read_tasks_from_file("src/serde.json")?;
            tasks.push(new_task);
            write_tasks_to_file("src/serde.json", &tasks)?;

            println!("{}", "Task adicionada com sucesso.".green());
        } else {
            println!("{}", "É necessário passar dois argumentos.".red());
        }
    }
    Ok(())
}

pub fn view_tasks(args: &Args) -> io::Result<()> {
    if let Some(view) = &args.view_tasks {
        if *view {
            let json_data = read_tasks_from_file("src/serde.json")?;
            println!("{}", "\n--------------------------\nTasks para concluir: \n--------------------------".bright_red());
            for (i, task) in json_data.iter().enumerate() {
                println!("{}", format!("#{}º", i + 1).white());
                println!("{}", format!("Nome: {}", task.name).green());
                println!("{}", format!("Prazo de conclusão: {}\n", task.completion_time).green());
            }
        }
    }
    Ok(())
}

pub fn remove_task(args: &Args) -> io::Result<()> {
    if let Some(task_to_remove) = &args.remove_task {
        let mut json_data = read_tasks_from_file("src/serde.json")?;
        json_data.retain(|task| task.name != *task_to_remove);

        write_tasks_to_file("src/serde.json", &json_data)?;

        println!("{}", "Task removida com sucesso.".green());
    }
    Ok(())
}

pub fn update_task_name(args: &Args) -> io::Result<()> {
    if let Some(task_to_update) = &args.update_name {
        if task_to_update.len() < 2 {
            println!("{}", "É necessário fornecer o nome atual e o novo nome da tarefa.".red());
            return Ok(());
        }

        let mut json_data = read_tasks_from_file("src/serde.json")?;
        let mut task_found = false;

        for task in json_data.iter_mut() {
            if task.name == task_to_update[0] {
                task.name = task_to_update[1].to_owned();
                task_found = true;
                break;
            }
        }
        if task_found {
            write_tasks_to_file("src/serde.json", &json_data)?;
            println!("{}", "Task atualizada com sucesso.".green());
        } else {
            println!("{}", "Task não encontrada.".red());
        }
    }
    Ok(())
}

pub fn update_task_completion_time(args: &Args) -> io::Result<()> {
    if let Some(task_to_update) = &args.update_concluded_time {
        if task_to_update.len() < 2 {
            println!("{}", "É necessário fornecer o nome da tarefa e o novo tempo de conclusão.".red());
            return Ok(());
        }

        let mut json_data = read_tasks_from_file("src/serde.json")?;
        let mut task_found = false;

        for task in json_data.iter_mut() {
            if task.name == task_to_update[0] {
                task.completion_time = task_to_update[1].to_owned();
                task_found = true;
                break;
            }
        }
        if task_found {
            write_tasks_to_file("src/serde.json", &json_data)?;
            println!("{}", "Task atualizada com sucesso.".green());
        } else {
            println!("{}", "Task não encontrada.".red());
        }
    }
    Ok(())
}

pub fn complete_task(args: &Args) -> io::Result<()> {
    if let Some(concluded_task) = &args.concluded_task {
        let mut json_file = read_tasks_from_file("src/serde.json")?;
        let mut completed_tasks = read_tasks_from_file("src/completed_tasks.json").unwrap_or_default();
        let mut task_found = false;

        json_file.retain(|task| {
            if task.name == *concluded_task {
                completed_tasks.push(task.clone());
                task_found = true;
                false // remove a tarefa do arquivo original
            } else {
                true //mantém a tarefa no arquivo original
            }
        });

        if task_found {
            write_tasks_to_file("src/serde.json", &json_file)?;
            write_tasks_to_file("src/completed_tasks.json", &completed_tasks)?;
            println!("{}", "Task adicionada a tarefas concluídas.".green());
        } else {
            println!("{}", "Task não encontrada.".red());
        }
    }
    Ok(())
}

pub fn view_concluded_tasks(args: &Args) -> io::Result<()> {
    if let Some(view) = &args.view_concluded_tasks {
        if *view {
            let json_data = read_tasks_from_file("src/completed_tasks.json")?;
            println!("{}", "\n--------------------------\nTasks já concluídas: \n--------------------------".bright_red());
            for (i, task) in json_data.iter().enumerate() {
                println!("{}", format!("#{}º", i + 1).white());
                println!("{}", format!("Nome: {}", task.name).green());
                println!("{}", format!("Prazo de conclusão: {}\n", task.completion_time).green());
            }
        }
    }
    Ok(())
}
