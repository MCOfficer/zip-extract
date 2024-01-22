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

All features are the same as the features of the [`zip`](https://github.com/zip-rs/zip/tree/v0.6.4#usage) crate:

- `aes-crypto`: Support for AES encryption via the the `zip` crate
- `deflate`: Support for the Deflate algorithm (`miniz_oxide` rust-backend)
- `deflate-miniz`: ^ dito (`miniz` C-backend)
- `deflate-zlib`: ^ dito (`zlib` C-backend)
- `unreserved`: Support for the `unreserved` feature of the `zip` crate
- `bzip2`: Support for .bzip2 archives via the `bzip2` crate
- `time`: Support for the `time` crate for the `zip` crate
- `zstd`: Support for the `zstd` crate for the `zip` crate
- `default`: enables `"aes-crypto", "bzip2", "deflate", "zstd"`
