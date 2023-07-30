use std::{fs, path::PathBuf, process::Command};

use crate::error::*;
use log::{debug, error, info, warn};

#[cfg(test)]
mod tests;

mod app;
mod config;
pub mod metadata;
mod paths;

// Re-Export
pub use app::*;
pub use config::*;
pub use paths::*;

/// The main struct of this crate.
///
/// It provides different helper functions to work with an [fdroid](https://f-droid.org/en/docs/) repository.
///
/// Create a new instance by running [`Repository::new`] and providing the path to the repository.
#[derive(Debug, Clone)]
pub struct Repository {
  /// absolute path of the /fdroid repository
  path: PathBuf,
}

impl Repository {
  /// Opens an existing [`Repository`] or creates a new one.
  /// The provided path is the directory, in which the fdroid repository will reside in.
  ///
  /// This function checks if the config file in [`Repository::config_path`] exists.
  /// If it does, it assumes an fdroid repository also exists.
  ///
  /// If no file is found, this function will initialize a new fdroid repository in the provided path
  /// by running [`Repository::initialize`].
  ///
  /// # Errors
  ///
  /// This function can return an error if:
  /// - the provided path is not a directory
  /// - initialization fails
  /// - updating the repository fails
  pub fn new(path: PathBuf) -> Result<Self> {
    if !path.is_dir() {
      return Err(Error::NotADirectory(path));
    }

    let repository = Self { path };

    // check if config.yml exists
    if !(repository.config_path().exists()) {
      // initialize directory
      repository.initialize()?;
    }

    Ok(repository)
  }

  /// Initializes a new repository
  ///
  /// # Error
  /// Returns an error if the command fails
  ///
  /// Runs `fdroid init`
  pub fn initialize(&self) -> Result<()> {
    info!("Initializing a new repository at {:?}!", self.path);

    self.run("init", &vec![]).map_err(|_| Error::Init)?;

    self.update()
  }

  /// Updates the repository
  ///
  /// Gets automatically called after every apk upload, metadata change, image upload, etc.
  /// and therefore **should never have to be called manually**.
  ///
  /// Runs `fdroid update -c; fdroid update`
  ///
  /// See [documentation](https://f-droid.org/en/docs/Setup_an_F-Droid_App_Repo/)
  pub fn update(&self) -> Result<()> {
    info!("Updating Repository");

    self.run("update", &vec!["-c"]).map_err(|_| Error::Update)?;
    self.run("update", &vec![]).map_err(|_| Error::Update)
  }

  /// Runs `fdroid publish`
  pub fn publish(&self) -> Result<()> {
    info!("Publishing Changes");

    self.run("publish", &vec![])
  }

  /// Deletes **all** apps and metadata (but keeps everything else)
  ///
  /// # Warning
  /// This function will permanently delete **ALL** files inside [`Repository::repo_path`] and [`Repository::metadata_path`]
  ///
  /// # Error
  /// Will return an error if:
  /// - removing/creating the directories fails
  /// - updating fails
  pub fn clear(&self) -> Result<()> {
    warn!("Clearing the repository!");

    // Delete all apps
    fs::remove_dir_all(self.repo_path())?;
    // Create directory again
    fs::create_dir(self.repo_path())?;

    // Delete all metadata files
    fs::remove_dir_all(self.metadata_path())?;
    // Create metadata directory
    fs::create_dir(self.metadata_path())?;

    // update index files etc
    self.update()
  }

  /// Cleans up metadata files but **does not** modify their data
  ///
  /// This function only prettifies the metadata files and does nothing else.
  ///
  /// It maybe has to be called if fields are missing inside the metadata files as this function also
  /// creates missing fields.
  ///
  /// Runs `fdroid rewritemeta`
  pub fn cleanup(&self) -> Result<()> {
    debug!("Cleaning up metadata files!");
    self.run("rewritemeta", &vec![])
  }

  /// Runs an fdroid command with the specified arguments
  fn run(&self, command: &str, args: &Vec<&str>) -> Result<()> {
    info!("Running command: \"fdroid {command}\" with arguemnts: \"{args:#?}\"");
    let run_result = Command::new("fdroid")
      .arg(command)
      .args(args)
      .current_dir(&self.path)
      .spawn()
      .map_err(|err| {
        debug!("Error spawning run command: {err:#?}");
        err
      })
      .ok()
      .and_then(|mut process| {
        process
          .wait()
          .map_err(|err| {
            debug!("Error while running process: {process:#?}");
            err
          })
          .ok()
      });

    if run_result.is_none() {
      let error_message =
        format!("Failed to run command: \"fdroid {command}\" with arguemnts: \"{args:#?}\"");
      error!("{}", error_message);
    }

    run_result.map(|_| ()).ok_or(Error::Run(
      format!("fdroid {command} {}", args.join(" "))
        .trim()
        .to_string(),
    ))
  }
}
