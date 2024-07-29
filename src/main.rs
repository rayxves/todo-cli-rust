pub mod todo_list;
use clap::Parser;
fn main() {
   let args = todo_list::Args::parse();
   todo_list::create_new_task(&args).expect("Erro ao criar nova task.");
}
