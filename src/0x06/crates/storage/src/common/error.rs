use crate::*;

pub type FsResult<T = ()> = core::result::Result<T, FsError>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FsError {
    /// The file was not found.
    FileNotFound,
    /// The file not in current sector.
    NotInSector,
    /// The end of the file was reached.
    EndOfFile,
    /// Writing to a file with no space left.
    WriteZero,
    /// The entry is not a directory.
    NotADirectory,
    /// The entry is not a file.
    NotAFile,
    /// The file is read-only.
    ReadOnly,
    /// Invalid operation.
    InvalidOperation,
    /// Not supported.
    NotSupported,
    /// Bad cluster.
    BadCluster,
    /// Invalid offset.
    InvalidOffset,
    /// The file name is invalid.
    FileNameError(FilenameError),
    /// Encountered an error while reading from the device.
    DeviceError(DeviceError),
    /// Invalid path.
    InvalidPath(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeviceError {
    /// The device is busy.
    Busy,
    /// Unknown device.
    UnknownDevice,
    /// Unknown error.
    Unknown,
    /// Invalid operation.
    InvalidOperation,
    /// Read error.
    ReadError,
    /// Write error.
    WriteError,
    /// The device error status code.
    WithStatus(usize),
}

/// Various filename related errors that can occur.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FilenameError {
    /// Tried to create a file with an invalid character.
    InvalidCharacter,
    /// Tried to create a file with no file name.
    FilenameEmpty,
    /// Given name was too long (we are limited to 8.3).
    NameTooLong,
    /// Can't start a file with a period, or after 8 characters.
    MisplacedPeriod,
    /// Can't extract utf8 from file name
    Utf8Error,
    /// Can't parse file entry
    UnableToParse,
}

impl From<FilenameError> for FsError {
    fn from(err: FilenameError) -> FsError {
        FsError::FileNameError(err)
    }
}

impl From<DeviceError> for FsError {
    fn from(err: DeviceError) -> FsError {
        FsError::DeviceError(err)
    }
}
