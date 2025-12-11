use std::fs;
use std::path::PathBuf;

pub fn get_data_dir() -> PathBuf {
    let mut dir = dirs::data_dir().unwrap_or(PathBuf::from("."));
    dir.push("rpgmaster");
    if !dir.exists() {
        fs::create_dir_all(&dir).unwrap();
    }
    dir
}