pub mod todo_list;
use clap::Parser;
fn main() {
   let args = todo_list::Args::parse();
   todo_list::create_new_task(&args).expect("Erro ao criar nova task.");
   todo_list::view_tasks(&args).expect("Não foi possível visualizar tasks");
   todo_list::remove_task(&args).expect("Não foi possível remover tasks.");
}
