extern crate tera;

use std::error;
use std::fmt;

use config::{Config};
use traces::{TraceMap, CoverageStat};


lazy_static! {
    static ref TERA: tera::Tera = compile_templates!("templates/**/*");
}


pub struct Report {
}

impl Report {

    pub fn render(config: &Config, traces: &TraceMap) -> Result<Self, Error> {
        Err(Error::Unknown)
    }

    pub fn export(&self, config: &Config) -> Result<(), Error> {
        Err(Error::Unknown)
    }
}


#[derive(Debug)]
pub enum Error {
    Unknown,
}

impl error::Error for Error {

    #[inline]
    fn description(&self) -> &str {
        ""
    }

    #[inline]
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

