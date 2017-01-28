//! This crate helps to read contents of a file with name taken from environment variable.
//!
//! Some applications have sensitive data, like API keys, which is unsafe to keep in VCS. One
//! possible solution is keeping secret data in files, taking their names from environment
//! variables. Say, you have to use cloud provider API key to control your PaaS system and you need
//! a key for external data API. You can then run your application like:
//!
//! ```ignore
//! $ CLOUD_API_KEY=/etc/secrets/cloud_api.key DATA_API_KEY=/etc/secrets/data_api.key your_app
//! ```
//!
//! The same pattern simplifies testing (with test keys and not production ones). It is extendable
//! too: you may have a default location for production use with ability to override it via
//! environment variables.

use std::convert;
use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::io::Read;


/// Main error type for this crate
#[derive(Debug)]
pub enum EnvFileError {
    /// Error happened while working with environment
    EnvVarError(env::VarError),
    /// Error happened while performing I/O
    Io(io::Error),
}

impl convert::From<env::VarError> for EnvFileError {
    fn from(err: env::VarError) -> EnvFileError {
        EnvFileError::EnvVarError(err)
    }
}

impl convert::From<io::Error> for EnvFileError {
    fn from(err: io::Error) -> EnvFileError {
        EnvFileError::Io(err)
    }
}

impl fmt::Display for EnvFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnvFileError::EnvVarError(ref err) => write!(f, "environment variable error: {}", err),
            EnvFileError::Io(ref err) => write!(f, "i/o error: {}", err),
        }
    }
}

impl error::Error for EnvFileError {
    fn description(&self) -> &str {
        match *self {
            EnvFileError::EnvVarError(ref err) => err.description(),
            EnvFileError::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EnvFileError::EnvVarError(ref err) => Some(err),
            EnvFileError::Io(ref err) => Some(err),
        }
    }
}


/// Read contents of a file which name is specified in `env_name` parameter
///
/// # Examples
///
/// ```rust,no_run
/// extern crate env_file;
///
/// let api_key = env_file::read("CLOUD_API_KEY").unwrap_or("default_key".to_string());
/// ```
pub fn read(env_name: &str) -> Result<String, EnvFileError> {
    let filename = try!(env::var(env_name));
    let mut file = try!(fs::File::open(filename));
    let mut retval = String::new();
    try!(file.read_to_string(&mut retval));
    Ok(retval)
}
