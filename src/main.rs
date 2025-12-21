mod db;
mod campaign;
mod error;
mod campaign_service;

use clap::{Parser, Subcommand, builder::OsStr};
use std::fs;
use error::AppError;
use campaign_service::{create_campaign, list_campaigns};
use campaign_service::load_campaign;

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
    LoadCampaign { name: String},
}

fn main() -> Result<(), AppError> {
    
    let cli = Cli::parse();
    let data_dir = db::get_data_dir();
    

    match cli.command {
        Commands::CreateCampaign { name } => {
            create_campaign(&data_dir, name)?;
            println!("Campaign created.");
        }
    
        Commands::ListCampaigns => {
            let campaigns = list_campaigns(&data_dir)?;
            for c in campaigns {
                print!("{}", c);
            }
        }

        Commands::LoadCampaign { name } =>{
            let campaign = load_campaign(&data_dir, &name)?;
            println!("Campaign {} loaded.", campaign.name);
        }
        
    }
    Ok(())
}