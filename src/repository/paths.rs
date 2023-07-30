//! Module only for getting specific paths

use crate::error::{Error, Result};
use crate::repository::Repository;
use std::fs;
use std::path::PathBuf;

impl Repository {
  /// Returns the path to the keystore file
  ///
  /// See [signing](https://f-droid.org/en/docs/Signing_Process/)
  pub fn keystore_path(&self) -> PathBuf {
    self.path.join("keystore.p12")
  }

  /// get the path to the config.yml file
  ///
  /// See [examples](https://gitlab.com/fdroid/fdroidserver/-/blob/master/examples/config.yml)
  pub fn config_path(&self) -> PathBuf {
    self.path.join("config.yml")
  }

  /// get the path of the metadata directory
  ///
  /// See [documentation](https://f-droid.org/en/docs/Build_Metadata_Reference/)
  pub fn metadata_path(&self) -> PathBuf {
    self.path.join("metadata")
  }

  /// gets the path to the unsigned files
  ///
  /// also creates the directory if it does not already exist
  ///
  /// See [documentation](https://f-droid.org/en/docs/Signing_Process/)
  pub fn unsigned_path(&self) -> Result<PathBuf> {
    let path = self.path.join("unsigned");

    // check if path is valid (could be invalid)
    if path.exists() {
      if path.is_dir() {
        Ok(path)
      } else {
        Err(Error::NotADirectory(path))
      }
    } else {
      fs::create_dir(path.clone())?;
      Ok(path)
    }
  }

  /// returns the path to the directory containing all apks (and some additional stuff)
  pub fn repo_path(&self) -> PathBuf {
    self.path.join("repo")
  }
}
