use super::*;

pub(super) struct RecordHeaderBuilder {
    key: KeyType,
}

impl RecordHeaderBuilder {
    pub(super) fn new() -> Self {
        Self {
            key: KeyType::default(),
        }
    }

    pub(super) fn with_key(mut self, k: KeyType) -> Self {
        self.key = k;
        self
    }

    pub(super) fn build(self) -> RecordHeader {
        let mut rh = RecordHeader::new(self.key.as_ref().to_owned(), 1, 1, 1);
        rh.set_created(1);
        rh
    }
}
