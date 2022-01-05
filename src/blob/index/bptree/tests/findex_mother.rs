use super::*;

pub(super) struct FindexMother {}

const SLEEP_TIME_MS: u64 = 100;

impl FindexMother {
    #[allow(dead_code)]
    // [0, 1, 2, 3, 4, ..., 99]
    pub(super) async fn create_default<FileIndexType: FileIndexTrait<KeyType>>(
        file: &str,
    ) -> FileIndexType {
        let findex = FileIndexBuilder::new()
            .with_path(PathBuf::from(file))
            .build()
            .await;
        tokio::time::sleep(std::time::Duration::from_millis(SLEEP_TIME_MS)).await;
        findex
    }

    // [0, 0, ...[REPS_AMOUNT times]..., 0, 1, 1, ..., (unique_amount - 1)]
    pub(super) async fn create_with_reps<
        FileIndexType: FileIndexTrait<KeyType>,
        const REPS_AMOUNT: usize,
    >(
        unique_amount: usize,
        file: &str,
    ) -> FileIndexType {
        let inmem = InMemoryIndexBuilder::new()
            .with_index_to_key_fn(|i| (i / REPS_AMOUNT).into())
            .with_to(unique_amount * REPS_AMOUNT)
            .build();
        let findex = FileIndexBuilder::new()
            .with_path(PathBuf::from(file))
            .with_recreate_index_file(true)
            .with_inmem(&inmem)
            .build()
            .await;
        tokio::time::sleep(std::time::Duration::from_millis(SLEEP_TIME_MS)).await;
        findex
    }

    // [offset, step + offset, step + 2 * offset, ..., step + (amount - 1) * offset]
    pub(super) async fn create_with_step_and_offset<
        FileIndexType: FileIndexTrait<KeyType>,
        const OFFSET: usize,
        const STEP: usize,
    >(
        amount: usize,
        file: &str,
    ) -> FileIndexType {
        let inmem = InMemoryIndexBuilder::new()
            .with_index_to_key_fn(|i| (100 * i + OFFSET).into())
            .with_to(OFFSET + amount)
            .build();
        let findex = FileIndexBuilder::new()
            .with_path(PathBuf::from(file))
            .with_recreate_index_file(true)
            .with_inmem(&inmem)
            .build()
            .await;
        tokio::time::sleep(std::time::Duration::from_millis(SLEEP_TIME_MS)).await;
        findex
    }
}
