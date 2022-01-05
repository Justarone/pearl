use super::*;

pub(super) struct InMemoryIndexBuilder {
    from: usize,
    to: usize,
    index_to_key_fn: fn(usize) -> KeyType,
}

impl InMemoryIndexBuilder {
    pub(super) fn new() -> Self {
        Self {
            from: 0,
            to: 100,
            index_to_key_fn: |i| i.into(),
        }
    }

    pub(super) fn with_from(mut self, from: usize) -> Self {
        self.from = from;
        self
    }

    pub(super) fn with_to(mut self, to: usize) -> Self {
        self.to = to;
        self
    }

    pub(super) fn with_index_to_key_fn(mut self, f: fn(usize) -> KeyType) -> Self {
        self.index_to_key_fn = f;
        self
    }

    pub(super) fn build(self) -> InMemoryIndex<KeyType> {
        let mut inmem = InMemoryIndex::<KeyType>::new();
        (self.from..self.to)
            .map(self.index_to_key_fn)
            .for_each(|key| {
                let rh = RecordHeaderBuilder::new().with_key(key.clone()).build();
                let entry = inmem.get_mut(&key);
                if let Some(entry) = entry {
                    entry.push(rh);
                } else {
                    inmem.insert(key, vec![rh]);
                }
            });
        inmem
    }
}
