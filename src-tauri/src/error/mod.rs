use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Json parse error: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("I/O error: {0}")]
    IO(#[from] io::Error),
    #[error("Excel error: {0}")]
    Excel(#[from] xlsxwriter::XlsxError),
}

impl serde::Serialize for ClientError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type Result<T> = std::result::Result<T, ClientError>;
