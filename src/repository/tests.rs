//! Module for Testing the library

use crate::repository::tests::utils::{get_repo_path, init_default, TestRepo};
use itertools::Zip;
use std::fs::File;
use std::io::Read;

/// Test Utils
mod utils {
  use crate::repository::Repository;
  use std::{fs, path::PathBuf};
  use uuid::Uuid;

  /// NewType for Repository struct
  /// Most important: Removes all files after being dropped
  pub struct TestRepo(Repository);

  impl Drop for TestRepo {
    fn drop(&mut self) {
      // delete directory
      fs::remove_dir_all(&self.0.path).unwrap();
    }
  }

  impl TestRepo {
    pub fn get_repo(&self) -> &Repository {
      &self.0
    }
  }

  impl Default for TestRepo {
    fn default() -> Self {
      // create new test repo in empty, random directory

      // create unique repo path
      let repo_path = get_repo_path().join(Uuid::new_v4().to_string());
      // create repo
      fs::create_dir_all(&repo_path).unwrap();

      Self(Repository::new(repo_path).unwrap())
    }
  }

  /// Returns the main path for test repos
  pub fn get_repo_path() -> PathBuf {
    PathBuf::from("development/tests")
      .canonicalize()
      .unwrap()
  }

  /// Returns a list of all available test apks
  pub fn get_test_apks() -> Vec<PathBuf> {
    vec![
      "com.dede.android_eggs_28",
      "fr.ralala.hexviewer_142",
      "me.hackerchick.catima_128",
      "nodomain.freeyourgadget.gadgetbridge_224",
      "org.woheller69.gpscockpit_240",
    ]
    .iter()
    .map(|name| {
      let file_path = get_repo_path().join(format!("../test-resources/{name}.apk"));
      file_path
    })
    .collect()
  }

  /// Returns a single TestApk for testing
  pub fn get_test_apk() -> PathBuf {
    get_test_apks().pop().unwrap()
  }

  /// Creates a new repo with one app uploaded
  pub fn init_default() -> TestRepo {
    let repo = TestRepo::default();

    // get app
    let test_apk = get_test_apk();

    // sign app
    repo.get_repo().sign_app(&test_apk).unwrap();
    repo
  }
}

/// Tests that in a new repo, all apps are empty
#[test]
fn apps_empty() {
  let repo = TestRepo::default();

  assert!(repo.get_repo().apps().unwrap().is_empty());
}

/// Test that uploading an app works
#[test]
fn upload_app() {
  let repo = init_default();

  // check that one app has been created
  let apps = repo.get_repo().apps().unwrap();
  assert_eq!(apps.len(), 1);
}

/// Test that getting and uploading config works
#[test]
fn upload_config() {
  let repo = TestRepo::default();

  // get default config
  let mut config = repo.get_repo().config().unwrap();

  // modify config
  config.repo_name = Some("new name".to_string());
  config.archive_description = Some("test description".to_string());

  // save new config
  repo.get_repo().set_config(&config).unwrap();

  // get new safed config
  let new_config = repo.get_repo().config().unwrap();

  assert_eq!(config, new_config);
}

/// Tests if signing works
#[test]
fn sign() {
  let repo = init_default();

  // check that one app has been created
  let apps = repo.get_repo().apps().unwrap();
  assert_eq!(apps.len(), 1);
}

/// Tests that deleting one app works
#[test]
fn delete_one() {
  let repo = init_default();

  let mut apps = repo.get_repo().apps().unwrap();

  // only one app should exist
  assert_eq!(apps.len(), 1);

  let mut app = apps.pop().unwrap();

  // only one package should exist
  assert_eq!(app.packages.len(), 1);

  let package = app.packages.pop().unwrap();

  // delete app
  repo.get_repo().delete_app(&package.apk_name).unwrap();

  // check that apps is empty
  let apps = repo.get_repo().apps().unwrap();

  assert!(apps.is_empty());
}

/// Tests that reading/writing metadata works
#[test]
fn metadata() {
  let repo = init_default();

  // get app
  let app = repo.get_repo().apps().unwrap().pop().unwrap();

  // get metadata
  let mut metadata = repo.get_repo().metadata(&app.package_name).unwrap();

  // change metadata
  metadata.AuthorName = Some("Dominik Schwaiger".to_string());
  metadata.AuthorEmail = Some("mail@dominik-schwaiger.ch".to_string());
  metadata.WebSite = Some("dominik-schwaiger.ch".to_string());

  // upload metadata
  repo
    .get_repo()
    .set_metadata(&app.package_name, &metadata)
    .unwrap();

  // get new metadata
  let new_metadata = repo.get_repo().metadata(&app.package_name).unwrap();

  assert_eq!(metadata, new_metadata);
}

/// Tests if uploading an image works
#[test]
fn image_upload() {
  let repo = TestRepo::default();

  // get new image
  let test_image_name = "test-icon.png";
  let image_path = get_repo_path()
    .join("../test-resources")
    .join(test_image_name);
  let mut image = File::open(&image_path).unwrap();

  // upload new image
  repo.get_repo().set_image(&image_path).unwrap();

  // get uploaded image
  let mut uploaded_image = File::open(repo.get_repo().image_path().unwrap()).unwrap();

  // get both image contents
  let mut image_content = vec![];
  image.read_to_end(&mut image_content).unwrap();

  let mut uploaded_image_content = vec![];
  uploaded_image
    .read_to_end(&mut uploaded_image_content)
    .unwrap();

  // check that lengths are the same
  assert_eq!(image_content.len(), uploaded_image_content.len());
  // check that length is bigger than 0
  assert!(!image_content.is_empty());

  // check that all elements are the same

  // content should be the same
  assert!(Zip::from((image_content, uploaded_image_content)).all(|zipped| zipped.0 == zipped.1));
}
