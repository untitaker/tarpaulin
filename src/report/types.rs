use std::fmt::{self, Display};
use std::result;

use config::Config;
use traces::TraceMap;


pub trait Report where Self: Sized {

    fn render(config: &Config, traces: &TraceMap) -> Result<Self>;
    fn export(&self, config: &Config);
}


pub type Result<A> = result::Result<A, Error>;


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    CoverallsAuth,
    Network,
    Unknown,
}

impl Default for Error {

    #[inline]
    fn default() -> Self {
        Error::Unknown
    }
}

impl Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Unknown => write!(f, "Unknown Error")
        }
    }
}

