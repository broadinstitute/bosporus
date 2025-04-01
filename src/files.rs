use std::fs;
use std::path::PathBuf;

pub(crate) fn config_dir() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".config");
        path.push("bosporus");
        fs::create_dir_all(&path).unwrap();
        path
    })
}

pub(crate) fn history_file() -> Option<PathBuf> {
    config_dir().map(|mut path| {
        path.push("history");
        fs::create_dir_all(&path).unwrap();
        path
    })
}