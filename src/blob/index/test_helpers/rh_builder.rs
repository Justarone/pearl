use super::*;

pub(crate) struct RecordHeaderBuilder {
    key: KeyType,
}

impl RecordHeaderBuilder {
    pub(crate) fn new() -> Self {
        Self {
            key: KeyType::default(),
        }
    }

    pub(crate) fn with_key(mut self, k: KeyType) -> Self {
        self.key = k;
        self
    }

    pub(crate) fn build(self) -> RecordHeader {
        let mut rh = RecordHeader::new(self.key.as_ref().to_owned(), 1, 1, 1);
        rh.set_created(1);
        rh
    }
}
