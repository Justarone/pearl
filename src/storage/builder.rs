use std::path::PathBuf;

use super::core::{Config, Error, Result, Storage};

/// `Builder` used for initializing a `Storage`.
#[derive(Default, Debug)]
pub struct Builder {
    config: Config,
}

impl Builder {
    /// Initializes the `Builder` with defaults
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates `Storage` based on given configuration
    pub fn build(self) -> Result<Storage> {
        if self.config.blob_file_name_prefix.is_none()
            || self.config.max_data_in_blob.is_none()
            || self.config.max_blob_size.is_none()
            || self.config.blob_file_name_prefix.is_none()
        {
            Err(Error::Unitialized)
        } else {
            Ok(Storage::new(self.config))
        }
    }

    /// # Description
    /// Sets a string with work dir as prefix for blob naming.
    /// If path doesn't exists, Storage will try to create it
    /// at initialization stage.
    /// # Examples
    /// ```
    /// let builder = Builder::new().work_dir("/tmp/pearl/");
    /// ```
    pub fn work_dir<S: Into<PathBuf>>(mut self, work_dir: S) -> Self {
        debug!("set work dir");
        let path: PathBuf = work_dir.into();
        debug!("work dir set to: {}", path.display());
        self.config.work_dir = Some(path);
        self
    }

    /// # Description
    /// Sets blob file max size
    /// Must be greater than zero
    /// # Examples
    /// ```
    /// let builder = Builder::new()
    ///     .work_dir("/tmp/pearl/")
    ///     .max_blob_size(1_000_000);
    /// ```
    pub fn max_blob_size(mut self, max_blob_size: u64) -> Self {
        if max_blob_size > 0 {
            self.config.max_blob_size = Some(max_blob_size);
            debug!(
                "maximum blob size set to: {}",
                self.config.max_blob_size.unwrap()
            );
        } else {
            error!("zero size blobs is useless, not set");
        }
        self
    }

    /// # Description
    /// Sets max number of records in single blob
    /// Must be greater than zero
    /// # Examples
    /// ```
    /// let builder = Builder::new()
    ///     .work_dir("/tmp/pearl/")
    ///     .max_blob_size(1_000_000)
    ///     .max_data_in_blob(1_000_000_000); // 1GB
    /// ```
    pub fn max_data_in_blob(mut self, max_data_in_blob: u64) -> Self {
        if max_data_in_blob > 0 {
            self.config.max_data_in_blob = Some(max_data_in_blob);
            info!(
                "max number of records in blob set to: {}",
                self.config.max_data_in_blob.unwrap()
            );
        } else {
            error!("zero size blobs is useless, not set");
        }
        self
    }

    /// # Description
    /// Sets blob file name prefix, e.g. if prefix set to `hellopearl`,
    /// files will be named as `hellopearl.[N].blob`.
    /// Where N - index number of file
    /// Must be not empty
    /// # Examples
    /// ```
    /// let builder = Builder::new()
    ///     .work_dir("/tmp/pearl/")
    ///     .max_blob_size(1_000_000)
    ///     .max_data_in_blob(1_000_000_000)
    ///     .blob_file_name_prefix("enough");
    /// ```
    pub fn blob_file_name_prefix<U: Into<String>>(mut self, blob_file_name_prefix: U) -> Self {
        let prefix = blob_file_name_prefix.into();
        if prefix.is_empty() {
            error!("passed empty file prefix, not set");
        } else {
            self.config.blob_file_name_prefix = Some(prefix);
            info!(
                "blob file format: {}.{{}}.blob",
                self.config.blob_file_name_prefix.as_ref().unwrap()
            );
        }
        self
    }
}
