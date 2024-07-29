use clap::Parser;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use colored::*;

#[derive(Debug, Parser)]
#[command(author = "Rayssa",
version = "1.0",
about = "Todo-list feito a partir de CLI.")]
pub struct Args {
    #[arg(short, long, help = "Adiciona uma nova task, o primeiro argumento deve ser o nome da task e o segundo deve ser a data de conclusão.", num_args(1..))]
    add_task: Option<Vec<String>>,

    #[arg(short, long, help = "Remove a task especificada")]
    remove_task: Option<String>,

    #[arg(short, long, help = "Muda uma variavél presente na task")]
    update_task: Option<String>,

    #[arg(short, long, help = "Imprime todas as taks não concluídas")]
    view_tasks: Option<bool>,

    #[arg(short = 'c', long, help = "Imprime todas as taks já concluídas")]
    view_concluded_tasks: Option<String>
}

#[derive(Serialize, Deserialize)]
struct NewTask {
    name: String,
    completion_time: String,
}

pub fn create_new_task(args: &Args) -> std::io::Result<()>{
    if let Some(task) = &args.add_task{
        if task.len() > 1{
            let data = NewTask{
                name: task[0].to_owned(),
                completion_time: task[1].to_owned(),
            };
            let data_serialized = serde_json::to_string_pretty(&data)?;
            fs::write("src/serde.json", data_serialized)?;
            println!("{}", format!("Task adcionada com sucesso.").green())
        } else {
            println!("{}", format!("É necessário passar dois argumentos.").red())
        }
    } else {
        println!("{}", format!("Nenhum argumento foi passado").red());
    }
    Ok(())
}

