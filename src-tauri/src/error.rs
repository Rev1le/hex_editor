use std::ffi::OsString;
use std::io;
use std::io::Error;

#[derive(Debug)]
pub enum HexEditorError {
    IoError(io::Error),
    OsError(OsString),
    PathError,
    StayFirstChunk,
    NoneBufferChunk
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