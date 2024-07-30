use clap::Parser;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::io::{self, Read, Write};
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

pub fn create_new_task(args: &Args) -> std::io::Result<()>{    if let Some(task) = &args.add_task{
        if task.len() > 1{
            let new_task = NewTask{
                name: task[0].to_owned(),
                completion_time: task[1].to_owned(),
            };

            let mut tasks: Vec<NewTask> = if let Ok(mut file) = fs::File::open("src/serde.json"){
                let mut data = String::new();
                file.read_to_string(&mut data)?;
                serde_json::from_str(&data).unwrap_or_else(|_| vec![]) //essa casinha é um closure, significa: se houver um erro ignore e retorne um vetor vazio
            } else {
                vec![]
            };

            tasks.push(new_task);

            let data_serialized = serde_json::to_string_pretty(&tasks)?;
            let mut file = fs::File::create("src/serde.json")?;
            file.write_all(data_serialized.as_bytes())?;
            println!("{}", format!("Task adcionada com sucesso.").green())
        } else {
            println!("{}", format!("É necessário passar dois argumentos.").red())
        }
    } 
    Ok(())
}

pub fn view_tasks(args: &Args) -> std::io::Result<()>{
    if let Some(view) =  &args.view_tasks{
        if *view {
            let mut json_file = fs::File::open("src/serde.json")?;
            let mut data = String::new();
            json_file.read_to_string(&mut data)?;
            let json_data: Vec<NewTask> = serde_json::from_str(&data).expect("Erro ao desserializar a data.");

            println!("{}", format!("\n--------------------------\nTasks para concluir: \n--------------------------").bright_red());
            for (i, task) in json_data.iter().enumerate() {
                println!("{}", format!("#{}º", i + 1).white());
                println!("{}", format!("Nome: {}", task.name).green());
                println!("{}", format!("Prazo de conclusão: {}\n", task.completion_time).green());
            }
        }
    }
    Ok(())
}

pub fn remove_task(args: &Args) -> std::io::Result<()> {
    if let Some(task_to_remove) = &args.remove_task{
        let mut json_file = fs::File::open("src/serde.json")?;
        let mut data = String::new();
        json_file.read_to_string(&mut data)?;
        let mut json_data: Vec<NewTask> = serde_json::from_str(&data).expect("Erro ao deserializar os dados para remover.");

        json_data.retain(|task| task.name != *task_to_remove); //retain filtra elementos que possuem determinada condicao, |item| condiiton

        let data_serialized = serde_json::to_string_pretty(&json_data)?;
        let mut file = fs::File::create("src/serde.json")?;
        file.write_all(data_serialized.as_bytes())?;


        println!("{}", format!("Tarefa removida com sucesso.").green());
    } 
    Ok(())
}

