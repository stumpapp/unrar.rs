pub use archive::Archive;
use unrar_sys as native;
pub mod archive;
pub mod error;
mod open_archive;
pub use open_archive::{
    CursorBeforeFile, CursorBeforeHeader, Extract, FileHeader, List, ListSplit, OpenArchive,
    VolumeInfo,
};
