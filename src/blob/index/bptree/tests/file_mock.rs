use crate::blob::FileTrait;
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
    async fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(FileImpl::new())),
        }
    }
}

#[async_trait::async_trait]
impl FileTrait for File {
    async fn open<TPath: AsRef<Path> + Send + Sync>(
        path: TPath,
        _ioring: Option<Rio>,
    ) -> IOResult<Self> {
        let str_path = path
            .as_ref()
            .to_str()
            .ok_or(IOError::from_raw_os_error(15))?;
        let storage_lock = SINGLETON_IN_MEMORY_STORAGE.read().unwrap();
        storage_lock
            .get(str_path)
            .map(|f| f.clone())
            .ok_or(IOError::from_raw_os_error(15))
    }

    async fn create<TPath: AsRef<Path> + Send + Sync>(
        path: TPath,
        _ioring: Option<Rio>,
    ) -> IOResult<Self> {
        let str_path = path
            .as_ref()
            .to_str()
            .ok_or(IOError::from_raw_os_error(15))?;
        let file = Self::new().await;
        let mut storage_lock = SINGLETON_IN_MEMORY_STORAGE.write().unwrap();
        storage_lock.insert(str_path.to_owned(), file.clone());
        Ok(file)
    }

    fn size(&self) -> u64 {
        self.inner.read().unwrap().size()
    }

    async fn write_append(&self, buf: &[u8]) -> IOResult<usize> {
        self.inner.write().unwrap().write_append(buf)
    }

    async fn write_append_sync(&self, buf: &[u8]) -> IOResult<usize> {
        self.inner.write().unwrap().write_append(buf)
    }

    async fn write_at(&self, offset: u64, buf: &[u8]) -> IOResult<usize> {
        self.inner.write().unwrap().write_at(buf, offset)
    }

    async fn read_all(&self) -> Result<Vec<u8>> {
        self.inner.read().unwrap().read_all()
    }

    async fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        self.inner.read().unwrap().read_at(buf, offset)
    }

    async fn read_at_sync(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        self.inner.read().unwrap().read_at(buf, offset)
    }

    async fn fsyncdata(&self) -> IOResult<()> {
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

    fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
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
        let end = std::cmp::min(offset + buf.len(), self.data.len());
        self.data
            .splice(offset..end, buf.iter().take(end - offset).copied());
        if end - offset < buf.len() {
            self.write_append(&buf[(end - offset)..])?;
        }
        Ok(buf.len())
    }

    fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        let offset = offset as usize;
        let read_end = std::cmp::min(self.data.len(), offset + buf.len());
        buf.iter_mut()
            .zip(self.data[offset..read_end].iter().copied())
            .for_each(|(d, s)| *d = s);
        Ok(read_end - offset)
    }

    fn read_all(&self) -> Result<Vec<u8>> {
        Ok(self.data.clone())
    }
}

struct FileImplMockBuilder {
    data: Vec<u8>,
}

impl FileImplMockBuilder {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn with_data(mut self, buf: Vec<u8>) -> Self {
        self.data = buf;
        self
    }

    fn build(self) -> FileImpl {
        let mut fi = FileImpl::new();
        fi.set_data(self.data);
        fi
    }
}

#[test]
fn file_mock_test_write_append() {
    let mut file = FileImplMockBuilder::new().build();

    file.write_append("hello".as_bytes()).unwrap();

    assert_eq!(file.data, "hello".as_bytes());
}

#[test]
fn file_mock_test_write_at() {
    let mut file = FileImplMockBuilder::new()
        .with_data("hello".as_bytes().to_owned())
        .build();

    file.write_at("hello".as_bytes(), 2).unwrap();

    assert_eq!(file.data, "hehello".as_bytes());
}

#[test]
fn file_mock_test_write_at_empty() {
    let mut file = FileImplMockBuilder::new().build();

    file.write_at("hello".as_bytes(), 0).unwrap();

    assert_eq!(file.data, "hello".as_bytes());
}

#[test]
fn file_mock_test_read_all() {
    let file = FileImplMockBuilder::new()
        .with_data("hello".as_bytes().to_owned())
        .build();

    let data = file.read_all().unwrap();

    assert_eq!(data, "hello".as_bytes());
}

#[test]
fn file_mock_test_read_at() {
    let file = FileImplMockBuilder::new()
        .with_data("hello".as_bytes().to_owned())
        .build();

    let mut data = [0; 3];
    file.read_at(&mut data, 1).unwrap();

    assert_eq!(data, "ell".as_bytes());
}

#[test]
fn file_mock_test_read_at_end() {
    let file = FileImplMockBuilder::new()
        .with_data("hello".as_bytes().to_owned())
        .build();

    let mut data = [0; 3];
    file.read_at(&mut data, 3).unwrap();

    assert_eq!(data, ['l' as u8, 'o' as u8, 0]);
}
