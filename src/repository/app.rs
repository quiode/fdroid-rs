//! # Manage [App]s and [Package]s
//!
//! Each [Repository] has multiple Apps.
//!
//! Each App has multiple Packages.
//!
//! A Package is a single [apk](https://en.wikipedia.org/wiki/Apk_(file_format))

use std::path::PathBuf;
use std::{
  fs::{self, File},
  io::Read,
};

use crate::aapt::*;
use crate::error::{Error, InvalidFile, Result};
use crate::metadata::Category;
use log::{info, warn};
use serde::{Deserialize, Serialize};

use super::Repository;

/// [DTO](https://en.wikipedia.org/wiki/Data_transfer_object) for a single app.
///
/// Get a List of all Apps by calling [Repository::apps].
#[derive(Clone, Serialize)]
pub struct App {
  /// the name of the package
  pub package_name: String,
  /// the categories of the package
  ///
  /// either a custom category or a category specified in [Category]
  pub categories: Vec<Category>,
  /// the suggested apk version to use
  pub suggested_version_code: String,
  /// the license of the repository (MIT, GPL, etc.)
  pub license: String,
  /// the name of the app
  pub name: String,
  /// when the app was added
  pub added: i64,
  /// when the app was last updated
  pub last_updated: i64,
  /// a list of all packages
  pub packages: Vec<Package>,
}

impl App {
  /// Reads a Json Value and tries to extract all fields to create a list of apps
  ///
  /// returns None if any field can't be converted
  fn from_json(value: &serde_json::Value) -> Option<Vec<Self>> {
    // get both lists
    let apps = value.get("apps")?;
    let packages = value.get("packages")?;

    let mut apps_vec = vec![];

    // map all app fields
    for app in apps.as_array()? {
      let name = app.get("name")?.as_str()?.to_owned();
      let suggested_version_code = app.get("suggestedVersionCode")?.as_str()?.to_owned();
      let license = app.get("license")?.as_str()?.to_owned();
      let package_name = app.get("packageName")?.as_str()?.to_owned();
      let last_updated = app.get("lastUpdated")?.as_i64()?.to_owned();
      let added = app.get("added")?.as_i64()?.to_owned();

      let mut categories = vec![];

      // get all categories (are saved in a map)
      for category in app.get("categories")?.as_array()? {
        categories.push(
          Category::deserialize(category)
            .unwrap_or(Category::Custom(category.as_str()?.to_string())),
        );
      }

      let mut packages_vec = vec![];

      let package = packages.get(&package_name)?;

      // map all package fields
      for package_entry in package.as_array()? {
        packages_vec.push(Package::from_json(package_entry)?);
      }

      apps_vec.push(App {
        name,
        suggested_version_code,
        license,
        package_name,
        last_updated,
        added,
        packages: packages_vec,
        categories,
      });
    }

    Some(apps_vec)
  }
}

/// [DTO](https://en.wikipedia.org/wiki/Data_transfer_object) for a specific version of a single app (So mostly an apk).
#[derive(Clone, Serialize)]
pub struct Package {
  // Exist
  pub added: i64,
  pub apk_name: String,
  pub hash: String,
  pub hash_type: String,
  pub package_name: String,
  pub size: u64,
  pub version_name: String,
  // Can be Missing
  pub nativecode: Vec<String>,
  pub max_sdk_version: Option<u32>,
  pub min_sdk_version: Option<u32>,
  pub sig: Option<String>,
  pub signer: Option<String>,
  pub target_sdk_version: Option<u32>,
  pub uses_permission: Vec<(String, Option<u32>)>,
  pub version_code: Option<u64>,
}

impl Package {
  /// Reads a Json Value and tries to extract all fields to create an instance of Package
  ///
  /// returns None if any field can't be converted
  fn from_json(value: &serde_json::Value) -> Option<Self> {
    // Always Exist
    let added = value.get("added")?.as_i64()?;
    let apk_name = value.get("apkName")?.as_str()?.to_owned();
    let hash = value.get("hash")?.as_str()?.to_owned();
    let hash_type = value.get("hashType")?.as_str()?.to_owned();
    let package_name = value.get("packageName")?.as_str()?.to_owned();
    let size = value.get("size")?.as_u64()?;
    let version_name = value.get("versionName")?.as_str()?.to_owned();

    // Can be missing
    let max_sdk_version = value
      .get("maxSdkVersion")
      .and_then(|val| val.as_u64())
      .and_then(|val| val.try_into().ok());

    let min_sdk_version = value
      .get("minSdkVersion")
      .and_then(|val| val.as_u64())
      .and_then(|val| val.try_into().ok());

    let mut nativecode = vec![];

    for nativecode_entry in value
      .get("nativecode")
      .and_then(|val| val.as_array())
      .unwrap_or(&vec![])
    {
      nativecode.push(nativecode_entry.as_str()?.to_owned());
    }

    let sig = value
      .get("sig")
      .and_then(|val| val.as_str())
      .map(|val| val.to_owned());

    let signer = value
      .get("signer")
      .and_then(|val| val.as_str())
      .map(|val| val.to_owned());

    let target_sdk_version = value
      .get("targetSdkVersion")
      .and_then(|val| val.as_u64())
      .and_then(|val| val.try_into().ok());

    let mut uses_permission = vec![];

    for uses_permission_entry in value
      .get("uses-permission")
      .unwrap_or(&serde_json::Value::Null)
      .as_array()
      .unwrap_or(&vec![])
    {
      uses_permission.push((
        uses_permission_entry.get(0)?.as_str()?.to_owned(),
        uses_permission_entry
          .get(1)?
          .as_i64()
          .and_then(|val| val.try_into().ok()),
      ));
    }

    let version_code = value.get("versionCode").and_then(|val| val.as_u64());

    Some(Self {
      added,
      apk_name,
      hash,
      hash_type,
      max_sdk_version,
      min_sdk_version,
      nativecode,
      package_name,
      sig,
      signer,
      size,
      target_sdk_version,
      uses_permission,
      version_code,
      version_name,
    })
  }
}

impl Repository {
  /// Reads the index file generated by fdroid and returns all apps
  ///
  /// Returns an error if the json file can't be mapped correctly
  pub fn apps(&self) -> Result<Vec<App>> {
    let index_file = self.repo_path().join("index-v1.json");

    if !index_file.exists() {
      // if no index file exists, no apps exist
      return Ok(vec![]);
    }

    let mut file = File::open(index_file)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    App::from_json(
      &serde_json::from_str(&file_content)
        .map_err(|_| Error::JsonConvert("Could not read repository index file!".to_owned()))?,
    )
    .ok_or(Error::JsonConvert(
      "Could not map repository index file!".to_owned(),
    ))
  }

  /// adds an app directly to the app repository
  pub fn add_app(&self, file_path: &PathBuf) -> Result<()> {
    info!("Adding new app: {file_path:?}");
    // save file
    let new_file_path = self.repo_path().join(
      file_path
        .file_name()
        .ok_or(Error::NotAFile(file_path.clone()))?,
    );

    // if file already exists, warn
    if new_file_path.exists() {
      warn!(
        "File already exists, overriding existing file: {:?}",
        new_file_path
      );
    }

    fs::copy(file_path, &new_file_path)?;

    // update meta data
    let update_result = self.update();

    // cleanup if error
    if update_result.is_err() && new_file_path.exists() && new_file_path.is_file() {
      fs::remove_file(new_file_path)?;
    }

    Ok(())
  }

  /// Deletes an apk (if it exists)
  pub fn delete_app(&self, apk_name: &str) -> Result<()> {
    warn!("Deleting \"{apk_name}\"");
    let file_path = self.repo_path().join(apk_name);

    // check if file exists
    if file_path.exists() {
      // check if file as really a file
      if file_path.is_file() {
        // delete the file
        fs::remove_file(file_path)?;

        // update metadata
        self.update()
      } else {
        Err(Error::NotAFile(file_path))
      }
    } else {
      warn!("Trying to delete \"{}\" but file does not exist!", apk_name);
      Ok(())
    }
  }

  /// Signs an apk and adds it
  ///
  /// - parses apk metadata
  /// - add apk to unsigned folder
  /// - signs apk
  pub fn sign_app(&self, file_path: &PathBuf) -> Result<()> {
    info!("Singing {file_path:?}");
    // get apk metadata
    let apk_metadata = get_apk_info(file_path)?;

    // get version and name
    let apk_version = get_version_code(&apk_metadata).ok_or(Error::InvalidFile(
      InvalidFile::with_reason(file_path.clone(), "Version Code not found!"),
    ))?;
    let apk_name = get_name(&apk_metadata).ok_or(Error::InvalidFile(InvalidFile::with_reason(
      file_path.clone(),
      "Name not found!",
    )))?;

    // Upload apk to unsigned folder
    let new_file_path = self
      .unsigned_path()?
      .join(format!("{}_{}.apk", apk_name, apk_version));

    fs::copy(file_path, new_file_path)?;

    // check if metadata exists
    let metadata = self.metadata(&apk_name);
    if metadata.is_err() {
      warn!("No metadata for this package exists, creating empty metadata file!");
      self.create_metadata(&apk_name)?;
    }

    // run fdroid publish
    self.publish()?;

    // run fdroid update
    self.update()?;

    Ok(())
  }
}
