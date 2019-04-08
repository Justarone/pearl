use futures::{
    future::Future,
    lock::{Mutex, MutexLockFuture},
    task::{Poll, Waker},
};
use std::{
    collections::BTreeMap,
    fs, io,
    os::unix::fs::FileExt,
    path::{Path, PathBuf},
    pin::Pin,
    sync::{
        atomic::{AtomicIsize, AtomicU64},
        Arc,
    },
};

use crate::record::{self, Record};

const BLOB_MAGIC_BYTE: u64 = 0xdeaf_abcd;

type Result<T> = std::result::Result<T, Error>;

/// A `Blob` struct for performing of database,
#[derive(Debug)]
pub struct Blob {
    header: Header,
    index: Index,
    name: FileName,
    file: fs::File,
    current_offset: Arc<Mutex<u64>>,
}

impl Clone for Blob {
    fn clone(&self) -> Self {
        Self {
            header: self.header.clone(),
            current_offset: self.current_offset.clone(),
            name: self.name.clone(),
            file: self.file.try_clone().unwrap(),
            index: self.index.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Index {
    headers: BTreeMap<Vec<u8>, RecordMetaData>,
}

#[derive(Debug, Default, Clone)]
struct RecordMetaData {
    blob_offset: u64,
    full_len: u64,
}

impl Index {
    fn new(_path: &Path) -> Self {
        // @TODO initialize new index from file
        Self {
            headers: BTreeMap::new(),
        }
    }

    fn from_default(_path: &Path) -> Result<Self> {
        // @TODO implement
        Ok(Default::default())
    }

    fn contains_key<K>(&self, key: K) -> bool
    where
        K: AsRef<[u8]>,
    {
        self.headers.contains_key(key.as_ref())
    }

    fn insert<K>(&mut self, key: K, meta: RecordMetaData)
    where
        K: AsRef<[u8]>,
    {
        self.headers.insert(key.as_ref().to_vec(), meta);
    }

    fn get<K>(&self, key: K) -> Option<&RecordMetaData>
    where
        K: AsRef<[u8]>,
    {
        self.headers.get(key.as_ref())
    }
}

/// Write future
#[derive(Debug)]
pub struct WriteFuture<'a> {
    file: fs::File,
    buf: Option<Vec<u8>>,
    offset: Arc<AtomicIsize>,
    write_position: MutexLockFuture<'a, AtomicU64>,
}

#[derive(Debug)]
pub struct ReadFuture {
    file: fs::File,
    key: Vec<u8>,
    loc: Location,
}

impl Future for ReadFuture {
    type Output = Result<Record>;

    fn poll(self: Pin<&mut Self>, _waker: &Waker) -> Poll<Self::Output> {
        let mut buf = vec![0u8; self.loc.size as usize];
        self.file.read_at(&mut buf, self.loc.offset).unwrap();
        let record = Record::from_raw(&buf).map_err(Error::RecordError)?;
        Poll::Ready(Ok(record))
    }
}

impl Blob {
    /// # Description
    /// Creates new blob file with given FileName
    /// # Panic
    /// Panics if file with same path already exists
    pub fn open_new(name: FileName) -> Result<Self> {
        let file = Self::create_file(&name.as_path())?;
        let mut blob = Self {
            header: Header::new(),
            file,
            index: Index::new(&name.as_path()),
            name,
            current_offset: Arc::new(Mutex::new(0)),
        };
        blob.flush()?;
        Ok(blob)
    }

    fn flush(&mut self) -> Result<()> {
        // @TODO implement
        Ok(())
    }

    fn create_file(path: &Path) -> Result<fs::File> {
        fs::OpenOptions::new()
            .create_new(true)
            // .append(true)
            .write(true)
            .read(true)
            .open(path)
            .map_err(Error::OpenNew)
    }

    fn open_file(path: &Path) -> Result<fs::File> {
        fs::OpenOptions::new()
            .create(false)
            .append(true)
            .read(true)
            .open(path)
            .map_err(Error::FromFile)
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn from_file(path: PathBuf) -> Result<Self> {
        let file = Self::open_file(&path)?;
        let name = FileName::from_path(&path)?;
        let len = file.metadata().map_err(Error::FromFile)?.len();
        let _header = Header::from_reader(&file)?;
        let index = Index::from_default(&path)?;

        let blob = Self {
            header: Header::new(),
            file,
            name,
            index,
            current_offset: Arc::new(Mutex::new(len)),
        };

        // @TODO Scan existing file to create index
        Ok(blob)
    }

    pub fn check_data_consistency(&self) -> Result<()> {
        // @TODO implement
        Ok(())
    }

    pub async fn write(&mut self, record: Record) -> Result<()> {
        let key = record.key().to_vec();
        if self.index.contains_key(&key) {
            return Err(Error::AlreadyContainsSameKey);
        }
        let buf = record.to_raw();
        let mut offset = await!(self.current_offset.lock());
        let bytes_written = self
            .file
            .write_at(&buf, *offset)
            .map_err(Error::WriteFailed)?;
        let meta = RecordMetaData {
            blob_offset: *offset,
            full_len: bytes_written as u64,
        };
        self.index.insert(key, meta);
        *offset += bytes_written as u64;
        Ok(())
    }

    pub async fn read(&self, key: Vec<u8>) -> Result<Record> {
        let read_fut = ReadFuture {
            file: self.file.try_clone().map_err(Error::CloneFd)?,
            loc: self.lookup(&key)?,
            key,
        };
        await!(read_fut)
    }

    fn lookup<K>(&self, key: &K) -> Result<Location>
    where
        K: AsRef<[u8]> + Ord,
    {
        let meta = self.index.get(key.as_ref()).ok_or(Error::NotFound)?;
        let offset = meta.blob_offset;
        Ok(Location::new(offset as u64, meta.full_len))
    }

    /// # Description
    // @TODO more useful result
    // pub fn flush(&mut self) -> Result<(), Error> {
    // @TODO implement
    //     Ok(())
    // }

    /// # Description
    /// Returns size of file in bytes
    // @TODO more useful result
    pub fn file_size(&self) -> Result<u64> {
        // @TODO implement
        Ok(0)
    }

    /// # Description
    /// Returns number of records in current blob
    // @TODO more useful result
    pub fn records_count(&self) -> Result<usize> {
        // @TODO implement
        Ok(0usize)
    }

    pub fn path(&self) -> PathBuf {
        self.name.as_path()
    }

    pub fn id(&self) -> usize {
        self.name.id
    }
}

#[derive(Debug)]
pub enum Error {
    OpenNew(io::Error),
    FromFile(io::Error),
    CloneFd(io::Error),
    NotFound,
    RecordFromRawFailed,
    PathWithoutFileName(PathBuf),
    NonUnicode(std::ffi::OsString),
    WrongFileNamePattern(PathBuf),
    IndexParseFailed(String),
    RecordError(record::Error),
    AlreadyContainsSameKey,
    DeserializationFailed(bincode::ErrorKind),
    WriteFailed(io::Error),
}

#[derive(Debug, Clone)]
pub struct FileName {
    name_prefix: String,
    id: usize,
    extension: String,
    dir: PathBuf,
}

impl FileName {
    pub fn new(name_prefix: String, id: usize, extension: String, dir: PathBuf) -> Self {
        Self {
            name_prefix,
            id,
            extension,
            dir,
        }
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        Self::try_from_path(path).ok_or_else(|| Error::WrongFileNamePattern(path.to_owned()))
    }

    pub fn as_path(&self) -> PathBuf {
        self.dir.join(self.name_to_string())
    }

    fn name_to_string(&self) -> String {
        format!("{}.{}.{}", self.name_prefix, self.id, self.extension)
    }

    fn try_from_path(path: &Path) -> Option<Self> {
        let extension = path.extension()?.to_str()?.to_owned();
        let stem = path.file_stem()?;
        let mut parts = stem
            .to_str()?
            .splitn(2, '.')
            .collect::<Vec<_>>()
            .into_iter();
        let name_prefix = parts.next()?.to_owned();
        let id = parts.next()?.parse().ok()?;
        let dir = path.parent()?.to_owned();
        Some(Self {
            name_prefix,
            id,
            extension,
            dir,
        })
    }
}

/// # Description
/// # Examples
#[derive(Debug, Clone)]
struct Header {
    magic_byte: u64,
    version: u32,
    flags: u64,
}

impl Header {
    pub fn new() -> Self {
        Self {
            magic_byte: BLOB_MAGIC_BYTE,
            version: 0,
            flags: 0,
        }
    }

    fn from_reader<R>(_reader: R) -> Result<Self>
    where
        R: std::io::Read,
    {
        // @TODO implement
        Ok(Self::new())
    }
}

#[derive(Debug)]
struct Location {
    offset: u64,
    size: u64,
}

impl Location {
    fn new(offset: u64, size: u64) -> Self {
        Self { offset, size }
    }
}
