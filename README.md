# zip-extract
[![CI](https://github.com/MCOfficer/zip-extract/workflows/CI/badge.svg)](https://github.com/MCOfficer/zip-extract/actions)
[![Crates.io](https://img.shields.io/crates/v/zip-extract)](https://crates.io/crates/zip-extract)
[![Docs.rs](https://docs.rs/zip-extract/badge.svg)](https://docs.rs/zip-extract/)

zip-extract's primary goal is simple: Automate tedious zip extraction. Ever wanted to just unpack
an archive somewhere? Well, here you go.

## Usage
```rust
let archive: Vec<u8> = download_my_archive()?;
let target_dir = PathBuf::from("my_target_dir"); // Doesn't need to exist

// The third parameter allows you to strip away toplevel directories.
// If `archive` contained a single directory, its contents would be extracted instead.
zip_extract::extract(Cursor::new(archive), &target_dir, true)?;
```


## Features
All features are passed through to `zip` and `flate2`. They are:

- `deflate`: Support for the Deflate algorithm (`miniz_oxide` rust-backend)
- `deflate-miniz`: ^ dito (`miniz` C-backend)
- `deflate-zlib`: ^ dito (`zlib` C-backend)
- `bzip2`: Support for .bzip2 archives via the `bzip2` crate

The default is `bzip2` and `deflate`.
