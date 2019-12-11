use super::prelude::*;

#[derive(Debug, Clone)]
pub(crate) struct Config {
    work_dir: Option<PathBuf>,
    max_blob_size: Option<u64>,
    max_data_in_blob: Option<u64>,
    blob_file_name_prefix: Option<String>,
    update_interval_ms: u64,
    allow_duplicates: bool,
}

// Getters
impl Config {
    pub fn work_dir(&self) -> Option<&Path> {
        self.work_dir.as_ref().map(AsRef::as_ref)
    }

    pub fn max_blob_size(&self) -> Option<u64> {
        self.max_blob_size
    }

    pub fn max_data_in_blob(&self) -> Option<u64> {
        self.max_data_in_blob
    }

    pub fn blob_file_name_prefix(&self) -> Option<&str> {
        self.blob_file_name_prefix.as_ref().map(AsRef::as_ref)
    }

    pub fn update_interval_ms(&self) -> u64 {
        self.update_interval_ms
    }

    pub fn allow_duplicates(&self) -> bool {
        self.allow_duplicates
    }
}

//Setters
impl Config {
    pub fn set_work_dir(&mut self, path: PathBuf) {
        self.work_dir = Some(path);
    }

    pub fn set_max_blob_size(&mut self, max_blob_size: u64) {
        self.max_blob_size = Some(max_blob_size);
    }

    pub fn set_max_data_in_blob(&mut self, max_data_in_blob: u64) {
        self.max_data_in_blob = Some(max_data_in_blob);
    }

    pub fn set_blob_file_name_prefix(&mut self, blob_file_name_prefix: String) {
        self.blob_file_name_prefix = Some(blob_file_name_prefix);
    }

    pub fn set_allow_duplicates(&mut self, allow_duplicates: bool) {
        self.allow_duplicates = allow_duplicates;
    }
}

// Impl Traits
impl Default for Config {
    fn default() -> Self {
        Self {
            work_dir: None,
            max_blob_size: None,
            max_data_in_blob: None,
            blob_file_name_prefix: None,
            update_interval_ms: 100,
            allow_duplicates: false,
        }
    }
}
