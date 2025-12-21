mod db;
mod campaign;

use clap::{Parser, Subcommand, builder::OsStr};
use std::fs;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateCampaign {
        name: String,
    },
    ListCampaigns,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use campaign::Campaign;
    use std::path::Path;
    
    let cli = Cli::parse();
    let data_dir = db::get_data_dir();
    

    match cli.command {
        Commands::CreateCampaign { name } => {
            let campaign = Campaign { name: name.clone() };

            let file_path = data_dir.join(format!("{}.json", name));
            let json = serde_json::to_string_pretty(&campaign)?;

            fs::write(file_path, json)?;
            println!("Campaign created!");
    }
        Commands::ListCampaigns => {
            for entry in fs::read_dir(&data_dir).unwrap() {
                let path = entry.unwrap().path();
                if path.extension() == Some(std::ffi::OsStr::new("json")) {
                    println!(
                        "{}",
                        path.file_stem().unwrap().to_string_lossy()
                    );
                }
            }
        }
    }
    Ok(())
}