mod db;

use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Save {text: String},
    Read,
}

fn main() {
    let cli = Cli::parse();
    let data_dir = db::get_data_dir();
    let file = data_dir.join("test.txt");

    match cli.command {
        Commands::Save { text } => {
            fs::write(&file, text).unwrap();
            println!("Arquivo salvo!");
        }
        Commands::Read => {
            let content = fs::read_to_string(&file).unwrap_or("empty".to_string());
            println!("Conteúdo: {}", content);
        }
    }
}