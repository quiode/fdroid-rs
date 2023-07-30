//! Module contain the package [Error] Enum

use core::fmt;
use std::path::PathBuf;
use std::{error, io};

/// [std::result::Result] type for [Error] for easier error handling
pub type Result<T> = std::result::Result<T, Error>;

/// Error enum for this crate
#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
  /// Gets thrown when a [`io::Error`] occurs
  File(io::Error),
  /// Gets thrown when the Deserialization of a YAML file fails
  ///
  /// Contains the [`serde_yaml::Error`]
  YAMLConvert(serde_yaml::Error),
  /// Gets thrown when the Deserialization of a Json file fails
  ///
  /// Contains an Error Message
  JsonConvert(String),
  /// Gets thrown when a Directory was expected but not provided
  ///
  /// Contains the invalid path
  NotADirectory(PathBuf),
  /// Gets thrown when a File was expected but not provided
  ///
  /// Contains the invalid path
  NotAFile(PathBuf),
  /// Gets thrown when Repository initialization fails
  Init,
  /// Gets thrown when [`crate::repository::Repository::update`] fails
  Update,
  /// Gets thrown when a command fails
  ///
  /// Contains the command
  Run(String),
  InvalidFile(InvalidFile),
}

/// Struct for an [Error::InvalidFile] error.
///
/// Has to contain the file but can optionally also contain the reason.
#[derive(Debug)]
pub struct InvalidFile {
  /// Optional reason as to why the file is invalid
  pub reason: Option<String>,
  /// Path to the invalid file
  pub file: PathBuf,
}

impl InvalidFile {
  /// Create a new Instance with empty reason
  pub fn without_reason(file: PathBuf) -> Self {
    Self { reason: None, file }
  }

  /// Create a new Instance with a reason
  pub fn with_reason(file: PathBuf, reason: &str) -> Self {
    Self {
      reason: Some(reason.to_string()),
      file,
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::File(err) => write!(f, "Error while working with a file: {err:#?}"),
      Error::YAMLConvert(err) => write!(f, "Error while converting a yml file: {err:#?}"),
      Error::JsonConvert(err) => write!(f, "Error while converting a json file: {err}"),
      Error::NotADirectory(path) => write!(f, "The provided path is not a directory: {path:?}"),
      Error::NotAFile(path) => write!(f, "The provided path is not a file: {path:?}"),
      Error::Init => write!(f, "Could not initialize the repository!"),
      Error::Update => write!(f, "Could not update the repository!"),
      Error::Run(command) => write!(f, "Command failed. Command \"{command}\"!"),
      Error::InvalidFile(invalid_file) => write!(
        f,
        "File with path {:?} is invalid.{}",
        invalid_file.file,
        invalid_file
          .reason
          .clone()
          .map(|reason| format!(" Reason: \"{reason}\"."))
          .unwrap_or(String::new())
      ),
    }
  }
}

impl error::Error for Error {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match self {
      Error::File(err) => Some(err),
      Error::YAMLConvert(err) => Some(err),
      _ => None,
    }
  }
}

impl From<io::Error> for Error {
  fn from(error: io::Error) -> Self {
    Self::File(error)
  }
}

impl From<serde_yaml::Error> for Error {
  fn from(error: serde_yaml::Error) -> Self {
    Self::YAMLConvert(error)
  }
}
