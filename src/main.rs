mod cli;
mod service;
mod storage;
mod todo;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    let mut todos = storage::load_todos();

    match cli.command {
        Commands::Add { title } => {
            service::add_todo(&mut todos, title);
            storage::save_todos(&todos);
            println!("Todo added");
        }
        Commands::List => {
            service::list_todos(&todos);
        }
        Commands::Done { id } => {
            if service::mark_done(&mut todos, id) {
                storage::save_todos(&todos);
                println!("Todo marked as done!");
            } else {
                println!("Todo not found");
            }
        }
        Commands::Delete { id } => {
            if service::delete_todo(&mut todos, id) {
                storage::save_todos(&todos);
                println!("Todo deleted!");
            } else {
                println!("Todo not found!");
            }
        }
    }
}
