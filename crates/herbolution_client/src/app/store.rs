use std::path::PathBuf;

use herbolution_lib::fs::Fs;
use lib::time::DeltaTime;

/// The data that persists through the entire duration of the application execution irrespective of the operating system or user directive.
///
/// This structure should only export data that is not dependent on the window, herbolution_engine, or navigation state.
pub struct Store {
    pub(crate) fs: Fs,
    pub(crate) delta_time: DeltaTime,
}

impl Store {
    /// Creates a new instance with the specified file system root path.
    pub fn new(root_dir: Option<PathBuf>) -> Self {
        let fs = Fs::new(root_dir);

        Self {
            fs,
            delta_time: DeltaTime::new(),
        }
    }
}
