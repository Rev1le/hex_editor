use std::ffi::OsString;
use std::io;

#[derive(Debug)]
pub enum HexEditorError {
    IoError(io::Error),
    OsError(OsString),
    PathError
}

impl From<std::io::Error> for HexEditorError {
    fn from(value: std::io::Error) -> Self {
        HexEditorError::IoError(value)
    }
}

impl From<OsString> for HexEditorError {
    fn from(value: OsString) -> Self {
        HexEditorError::OsError(value)
    }
}