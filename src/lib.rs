//! # Fdroid
//! Create and manipulate an [fdroid](https://f-droid.org/) repository.
//!
//! If you want to edit the metadata of a specific package, see [metadata] for the structs.
//! If you want to configure the repository, see [Config].
//! If you want to edit an app, see [App].
//!
//! All of these structs can be centrally edited using the methods of [Repository].
//!
//! ## Quickstart
//! For an in-depth documentation, go to [Repository].
//! ### Create a new repository
//! ```no_run
//! # use std::path::PathBuf;
//! # use fdroid::Repository;
//! # let repository_path = PathBuf::from("/fdroid");
//! let repository = Repository::new(repository_path).unwrap();
//! ```
//! `repository_path` has to be a valid path pointing to an empty directory or an already existing repository.
//!
//! If the repository does not exists, it will create a new one.
//!
//! ### Get all current apps
//! ```no_run
//! # use std::path::PathBuf;
//! # use fdroid::Repository;
//! # let repository_path = PathBuf::from("/fdroid");
//! # let repository = Repository::new(repository_path).unwrap();
//! let apps = repository.apps().unwrap();
//! ```
//!
//! ### Upload a new app
//! ```no_run
//! # use std::path::PathBuf;
//! # use fdroid::Repository;
//! # let repository_path = PathBuf::from("/fdroid");
//! # let repository = Repository::new(repository_path).unwrap();
//! # let app_path = PathBuf::from("/apps/app-1");
//! repository.add_app(&app_path).unwrap();
//! ```
//! ## External Dependencies
//! - [fdroidserver](https://gitlab.com/fdroid/fdroidserver)  
//! For working with the repository itself
//! - [android-sdk-build-tools](https://developer.android.com/tools/releases/build-tools)  
//! Uses [aapt](https://elinux.org/Android_aapt) for extracting metadata from apks
//!
//! ## Logging
//! This crate uses the [log crate](https://docs.rs/log/latest/log/) to log all **write** changes.

mod aapt;
pub mod error;
mod repository;

// Re-Export
pub use repository::*;
