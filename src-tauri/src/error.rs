use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("System error: {0}")]
    System(String),
    #[error("Web Analyzer module error: {0}")]
    ModuleFailed(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
