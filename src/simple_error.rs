use std::{error::Error, fmt::{self, Display}};

#[derive(Debug)]
pub struct SimpleError {
    src: Option<Box<dyn Error>>,
    desc: String
}

impl SimpleError {
    pub fn new(src: Option<Box<dyn Error>>, desc: &str) -> SimpleError {
        SimpleError {
            src, 
            desc: desc.to_string()
        }
    }
}

impl Error for SimpleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.src.as_ref().map(|v| v.as_ref())
    }

    fn description(&self) -> &str {
        return &(self.desc);
    }
}

impl Display for SimpleError {
    fn fmt(self: &SimpleError, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {}", self.desc)
    }
}
