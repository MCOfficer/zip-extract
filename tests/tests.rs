use cute_log::log::LevelFilter;
use cute_log::Logger;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Once;
use zip::result::ZipError::InvalidArchive;
use zip_extract::ZipExtractError::Zip;
use zip_extract::{extract, ZipExtractError};

static INIT: Once = Once::new();

fn initialize() {
    INIT.call_once(|| {
        const LOGGER: Logger = Logger::new();
        LOGGER.set_max_level(LevelFilter::Debug);
        LOGGER.set_logger().unwrap();
    });
}

#[test]
fn valid_archive() {
    initialize();

    let mut source = vec![];
    source.extend_from_slice(include_bytes!("data/valid.zip"));
    let mut target = get_tempdir();
    target.push("test"); // let zip-extract create it

    extract(Cursor::new(source), &target, true).unwrap();
    assert!(!dir_diff::is_different(target, "tests/data/valid").unwrap());
}

#[test]
fn valid_archive_with_toplevel() {
    initialize();

    let mut source = vec![];
    source.extend_from_slice(include_bytes!("data/valid_toplevel.zip"));
    let mut target = get_tempdir();
    target.push("test"); // let zip-extract create it

    extract(Cursor::new(source), &target, true).unwrap();

    assert!(!dir_diff::is_different(target, "tests/data/valid").unwrap());
}

#[test]
fn valid_archive_forbid_toplevel() {
    initialize();

    let mut source = vec![];
    source.extend_from_slice(include_bytes!("data/valid_toplevel.zip"));
    let mut target = get_tempdir();
    target.push("test"); // let zip-extract create it

    extract(Cursor::new(source), &target, false).unwrap();

    target.push("valid");
    assert!(!dir_diff::is_different(target, "tests/data/valid").unwrap());
}

#[test]
fn invalid_target() {
    initialize();

    let e = extract(Cursor::new(Vec::new()), &PathBuf::default(), false);
    assert!(matches!(e, Err(ZipExtractError::Io(..))))
}

#[test]
fn invalid_archive() {
    initialize();

    let mut source = vec![];
    source.extend_from_slice(include_bytes!("data/invalid.zip"));
    let e = extract(Cursor::new(source), &PathBuf::from("."), false);
    assert!(matches!(e, Err(Zip(InvalidArchive(..)))));
}

fn get_tempdir() -> PathBuf {
    tempfile::Builder::new()
        .prefix("zip-extract.")
        .rand_bytes(12)
        .tempdir()
        .unwrap()
        .into_path()
}
