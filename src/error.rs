use std::fmt;

#[derive(Debug)]
pub enum AppError{
    CampaignAlreadyExists(String),
    CampaignNotFound,
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for AppError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self {
            AppError::CampaignAlreadyExists(name) => {
                write!(f, "Campaign {} already exists.", name)
            }
            AppError::CampaignNotFound => {
                write!(f, "Campaign not found.")
            }
            AppError::Io(_) => {
                write!(f, "Erro ao acessar o sistema de arquivos.")
            }
            AppError::Json(_) => {
                write!(f, "Erro ao processar dados da campanha.")
            }
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Json(err)
    }
}