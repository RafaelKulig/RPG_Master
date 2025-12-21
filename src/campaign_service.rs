use std::fs;
use std::path::PathBuf;

use crate::campaign::Campaign;
use crate::error::AppError;

pub fn load_campaign(
    data_dir: &PathBuf,
    name: &str,
) -> Result<Campaign, AppError> {
    let file_path = data_dir.join(format!("{}.json", name));

    if !file_path.exists() {
        return Err(AppError::CampaignNotFound);
    }

    let content = fs::read_to_string(file_path)?;
    let campaign: Campaign = serde_json::from_str(&content)?;

    Ok(campaign)
}

pub fn create_campaign (data_dir: &PathBuf, name: String) -> Result<(),AppError>{
    let file_path = data_dir.join(format!("{}.json", name));

    if file_path.exists() {
        return Err(AppError::CampaignAlreadyExists(name));
    }

    let campaign = Campaign{ name };
    let json = serde_json::to_string_pretty(&campaign)?;
    fs::write(file_path,json)?;

    Ok(())
}

pub fn list_campaigns(data_dir: &PathBuf) -> Result<Vec<String>, AppError> {
    let mut campaigns = Vec::new();
    for entry in fs::read_dir(data_dir)? {
        let path = entry?.path();
        if path.extension() == Some(std::ffi::OsStr::new("json")){
            if let Some(name) = path.file_stem() {
                campaigns.push(name.to_string_lossy().to_string());
            }
        }
    }

    if campaigns.is_empty() {
        return Err(AppError::CampaignNotFound);
    }

    Ok(campaigns)
}