# fdroid
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]
[![Publish Status][publish-badge]][publish-url]
[![docs.rs][docs-badge]][docs-url]


[crates-badge]: https://img.shields.io/crates/v/fdroid.svg
[crates-url]: https://crates.io/crates/fdroid
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/quiode/fdroid-rs/blob/master/LICENSE
[actions-badge]: https://github.com/quiode/fdroid-rs/workflows/Check/badge.svg
[actions-url]: https://github.com/quiode/fdroid-rs/actions?query=workflow%3ACheck+branch%3Amaster
[publish-badge]: https://github.com/quiode/fdroid-rs/workflows/Publish/badge.svg
[publish-url]: https://github.com/quiode/fdroid-rs/actions?query=workflow%3APublish+branch%3Amaster
[docs-badge]: https://img.shields.io/docsrs/fdroid/latest
[docs-url]: https://docs.rs/fdroid

Create and manipulate an [fdroid](https://f-droid.org/) repository.

Uses the [fdroidserver](https://gitlab.com/fdroid/fdroidserver) tool internally.

For documentation, see the [docs.rs](http://docs.rs/fdroid) page

## Dependencies
- [fdroidserver](https://gitlab.com/fdroid/fdroidserver)  
For working with the repository itself
- [android-sdk-build-tools](https://developer.android.com/tools/releases/build-tools)  
Uses [aapt](https://elinux.org/Android_aapt) for extracting metadata from apks