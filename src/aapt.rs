//! Module for working with [aapt](https://stackoverflow.com/questions/28234671/what-is-aapt-android-asset-packaging-tool-and-how-does-it-work)

use std::{path::PathBuf, process::Command};

use regex::Regex;

use crate::error::{Error, InvalidFile, Result};

/// Returns metadata of an apk as a string
///
/// # Error
/// Returns an error if the file does not exist or can't be parsed
pub fn get_apk_info(apk_path: &PathBuf) -> Result<String> {
  if apk_path.is_file() {
    // run aapt command
    let output = Command::new("aapt")
      .arg("dump")
      .arg("badging")
      .arg(apk_path)
      .output()
      .map_err(|_| Error::InvalidFile(InvalidFile::without_reason(apk_path.clone())))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
  } else {
    Err(Error::NotAFile(apk_path.clone()))
  }
}

/// gets the version code from an apk metadata string
///
/// returns [None] if the version code couldn't be found
pub fn get_version_code(metadata: &str) -> Option<u32> {
  let regex = Regex::new(r"versionCode='(\d+)'").unwrap();

  // apply regext to string
  let captures = regex.captures(metadata)?;

  let string_version_code = captures.get(1)?;

  string_version_code.as_str().parse().ok()
}

/// gets the name from an apk metadata string
///
/// returns [None] if the name couldn't be found
pub fn get_name(metadata: &str) -> Option<String> {
  let regex = Regex::new(r"name='((?:[[:alpha:]]|\.)+)'").unwrap();

  // apply regext to string
  let captures = regex.captures(metadata)?;

  let name = captures.get(1)?;

  Some(name.as_str().to_string())
}
