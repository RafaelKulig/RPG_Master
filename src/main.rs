mod db;
mod campaign;
mod error;

use clap::{Parser, Subcommand, builder::OsStr};
use std::fs;
use error::AppError;

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

fn main() -> Result<(), AppError> {
    use campaign::Campaign;
    use std::path::Path;
    
    let cli = Cli::parse();
    let data_dir = db::get_data_dir();
    

    match cli.command {
        Commands::CreateCampaign { name } => {

            let file_path = data_dir.join(format!("{}.json", name));
            if file_path.exists() {
                return Err(AppError::CampaignAlreadyExists(name));
            }
    }
        Commands::ListCampaigns => {
            let mut found = false;

            for entry in fs::read_dir(&data_dir)? {
                let path = entry?.path();
                if path.extension() == Some(std::ffi::OsStr::new("json")) {
                    found = true;
                    println!("{}", path.file_stem().unwrap().to_string_lossy());
                }
            }
            if !found{
                return  Err(AppError::CampaignNotFound);
            }
        }
    }
    Ok(())
}