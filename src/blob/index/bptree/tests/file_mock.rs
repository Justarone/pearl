use crate::prelude::*;
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::sync::RwLock;

type GlobalFileStorage = Arc<RwLock<BTreeMap<String, File>>>;

lazy_static! {
    static ref SINGLETON_IN_MEMORY_STORAGE: GlobalFileStorage =
        Arc::new(RwLock::new(BTreeMap::new()));
}

#[derive(Debug, Clone)]
pub struct File {
    inner: Arc<RwLock<FileImpl>>,
}

impl File {
    fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(FileImpl::new())),
        }
    }

    pub(crate) async fn open(path: impl AsRef<Path>, ioring: Option<Rio>) -> IOResult<Self> {
        let str_path_option = path.as_ref().to_str();
        if let None = str_path_option {
            return Err(IOError::from_raw_os_error(15));
        }
        // None case checked
        let str_path = str_path_option.unwrap();
        let storage_lock = SINGLETON_IN_MEMORY_STORAGE.read().unwrap();
        let entry = storage_lock.get(str_path);
        if let Some(file) = entry {
            Ok(file.clone())
        } else {
            // file not found
            Err(IOError::from_raw_os_error(15))
        }
    }

    pub(crate) async fn create(path: impl AsRef<Path>, ioring: Option<Rio>) -> IOResult<Self> {
        let str_path_option = path.as_ref().to_str();
        if let None = str_path_option {
            return Err(IOError::from_raw_os_error(15));
        }
        // None case checked
        let str_path = str_path_option.unwrap();
        let mut storage_lock = SINGLETON_IN_MEMORY_STORAGE.write().unwrap();
        let file = Self::new();
        storage_lock.insert(str_path.to_owned(), file.clone());
        Ok(file)
    }

    pub fn size(&self) -> u64 {
        self.inner.read().unwrap().size()
    }

    pub(crate) async fn write_append(&self, buf: &[u8]) -> IOResult<usize> {
        self.inner.write().unwrap().write_append(buf)
    }

    pub(crate) async fn write_append_sync(&self, buf: &[u8]) -> IOResult<usize> {
        self.inner.write().unwrap().write_append(buf)
    }

    pub(crate) async fn write_at(&self, offset: u64, buf: &[u8]) -> IOResult<usize> {
        self.inner.write().unwrap().write_at(buf, offset)
    }

    pub(crate) async fn read_all(&self) -> Result<Vec<u8>> {
        self.inner.read().unwrap().read_all()
    }

    pub(crate) async fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        self.inner.read().unwrap().read_at(buf, offset)
    }

    pub(crate) async fn read_at_sync(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        self.inner.read().unwrap().read_at(buf, offset)
    }

    pub(crate) async fn fsyncdata(&self) -> IOResult<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct FileImpl {
    data: Vec<u8>,
}

impl FileImpl {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn size(&self) -> u64 {
        self.data.len() as u64
    }

    fn write_append(&mut self, buf: &[u8]) -> IOResult<usize> {
        self.data.extend(buf.iter());
        Ok(buf.len())
    }

    fn write_at(&mut self, buf: &[u8], offset: u64) -> IOResult<usize> {
        let offset = offset as usize;
        self.data.splice(offset..offset, buf.iter().copied());
        Ok(buf.len())
    }

    fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        let offset = offset as usize;
        buf.clone_from_slice(&self.data[offset..]);
        Ok(self.data.len() - offset)
    }

    fn read_all(&self) -> Result<Vec<u8>> {
        Ok(self.data.clone())
    }
}
