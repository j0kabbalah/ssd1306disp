use anyhow::anyhow;
use display_interface::DisplayError;
use procfs::ProcError;
use std::{fmt::Display, path::PathBuf};
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

impl From<ProcError> for Error {
    fn from(value: ProcError) -> Self {
        let err = match value {
            ProcError::Incomplete(path) => {
                let spath = pathbuf_to_string(path);
                format!("Incomplete: at {}", spath)
            }
            ProcError::InternalError(err) => format!("InternalError: {}", err),
            ProcError::Io(err, path) => {
                let spath = pathbuf_to_string(path);
                format!("IO: {} at {}", err, spath)
            }
            ProcError::NotFound(path) => {
                let spath = pathbuf_to_string(path);
                format!("NotFound: at {}", spath)
            }
            ProcError::Other(msg) => format!("Other: {}", msg),
            _ => "unknown".to_owned(),
        };
        let msg = "ProcError: ".to_owned() + &err;
        let source = anyhow!(msg.clone());
        Error { msg, source }
    }
}

fn pathbuf_to_string(path: Option<PathBuf>) -> String {
    path.map(|p| p.into_os_string().into_string().unwrap_or("???".to_owned()))
        .unwrap_or("---".to_owned())
}
