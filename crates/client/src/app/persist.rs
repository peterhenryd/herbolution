use std::path::PathBuf;

use lib::fs::Fs;
use lib::time::DeltaTime;

/// The data that persists through the entire duration of the application execution irrespective of the operating system or user directive.
///
/// This structure should only export data that is not dependent on the window, engine, or navigation state.
pub struct Persist {
    pub(crate) fs: Fs,
    pub(crate) delta_time: DeltaTime,
}

impl Persist {
    /// Creates a new instance with the specified file system root path.
    pub fn new(root_path: PathBuf) -> Self {
        Self { fs: Fs::new(root_path), delta_time: DeltaTime::new() }
    }
}
