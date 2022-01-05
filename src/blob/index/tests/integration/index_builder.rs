use super::*;
use std::ops::Range;

pub(super) struct IndexBuilder {
    dump_to_file: bool,
    id: usize,
    dir: PathBuf,
    recreate_index_file: bool,
    keys_range_to_append: Option<Range<usize>>,
    key_map: fn(usize) -> usize,
}

impl IndexBuilder {
    pub(super) fn new() -> Self {
        Self {
            dump_to_file: false,
            id: 0,
            dir: PathBuf::from("/tmp/pearl_integration"),
            recreate_index_file: false,
            keys_range_to_append: None,
            key_map: |k| k,
        }
    }

    pub(super) fn with_dump_to_file(mut self, dump_to_file: bool) -> Self {
        self.dump_to_file = dump_to_file;
        self
    }

    pub(super) fn with_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub(super) fn with_keys_range_to_append(mut self, range: Range<usize>) -> Self {
        self.keys_range_to_append = Some(range);
        self
    }

    pub(super) fn with_key_map(mut self, key_map: fn(usize) -> usize) -> Self {
        self.key_map = key_map;
        self
    }

    pub(super) fn insert_keys(
        key_map: fn(usize) -> usize,
        index: &mut TestIndex,
        range: Range<usize>,
    ) {
        range
            .map(|k| key_map(k))
            .map(|k| RecordHeaderBuilder::new().with_key(k.into()).build())
            .for_each(|rh| index.push(rh).unwrap());
    }

    pub(super) async fn build(self) -> TestIndex {
        let filename = FileName::new("file".to_owned(), self.id, "index".to_owned(), self.dir);
        let mut index = TestIndex::new(
            filename,
            None,
            IndexConfig {
                recreate_index_file: self.recreate_index_file,
                bloom_config: None,
            },
        );

        if let Some(range) = self.keys_range_to_append {
            Self::insert_keys(self.key_map, &mut index, range);
        }

        if self.dump_to_file {
            index.dump().await.expect("Can't dump to file");
        }

        index
    }
}
