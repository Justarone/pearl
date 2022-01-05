use super::*;

pub(crate) struct FileIndexBuilder<'inmem_ref, FileIndexType: FileIndexTrait<KeyType>> {
    inmem: Option<&'inmem_ref InMemoryIndex<KeyType>>,
    meta: Vec<u8>,
    path: PathBuf,
    rio: Option<Rio>,
    recreate_index_file: bool,
    marker: PhantomData<FileIndexType>,
}

impl<'inmem_ref, FileIndexType: FileIndexTrait<KeyType>>
    FileIndexBuilder<'inmem_ref, FileIndexType>
{
    pub(crate) fn new() -> Self {
        Self {
            inmem: None,
            meta: vec![META_VALUE; META_SIZE],
            path: PathBuf::from("/tmp/bptree_index.bin"),
            marker: PhantomData::<FileIndexType>,
            recreate_index_file: true,
            rio: None,
        }
    }

    pub(crate) fn with_inmem(mut self, inmem: &'inmem_ref InMemoryIndex<KeyType>) -> Self {
        self.inmem = Some(inmem);
        self
    }

    #[allow(dead_code)]
    pub(crate) fn with_meta(mut self, meta: Vec<u8>) -> Self {
        self.meta = meta;
        self
    }

    #[allow(dead_code)]
    pub(crate) fn with_rio(mut self, rio: Rio) -> Self {
        self.rio = Some(rio);
        self
    }

    pub(crate) fn with_recreate_index_file(mut self, recreate_index_file: bool) -> Self {
        self.recreate_index_file = recreate_index_file;
        self
    }

    pub(crate) fn with_path(mut self, path: PathBuf) -> Self {
        self.path = path;
        self
    }

    pub(crate) async fn build(self) -> FileIndexType {
        let default_inmem = InMemoryIndexBuilder::new().build();
        FileIndexType::from_records(
            &self.path,
            self.rio,
            self.inmem.unwrap_or(&default_inmem),
            self.meta,
            self.recreate_index_file,
        )
        .await
        .expect("Can't create file index")
    }
}
