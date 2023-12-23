use anyhow::anyhow;
use display_interface::DisplayError;
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub struct Error {
    msg: String,
    source: anyhow::Error,
}

impl Error {
    #![allow(dead_code)]
    fn new(msg: &str, src: anyhow::Error) -> Self {
        Error {
            msg: msg.to_owned(),
            source: src,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.msg)
    }
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Error {
            msg: value.to_string(),
            source: value,
        }
    }
}

impl From<DisplayError> for Error {
    fn from(value: DisplayError) -> Self {
        let err = match value {
            DisplayError::BusWriteError => "BusWriteError",
            DisplayError::CSError => "CSError",
            DisplayError::DCError => "DCError",
            DisplayError::DataFormatNotImplemented => "DataFormatNotImplemented",
            DisplayError::InvalidFormatError => "InvalidFormatError",
            DisplayError::OutOfBoundsError => "OutOfBoundsError",
            DisplayError::RSError => "RSError",
            _ => "unknown",
        };
        let msg = "DisplayError: ".to_owned() + err;
        let source = anyhow!(msg.clone());
        Error { msg, source }
    }
}
