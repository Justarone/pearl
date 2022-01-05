use super::core::IndexStruct;
use super::*;
use std::ops::Range;

mod index_builder;
use index_builder::IndexBuilder;

type TestIndex = IndexStruct<BPTreeFileIndex<KeyType, file_mock::File>, KeyType>;

#[tokio::test]
async fn check_count_after_load_index_from_disk() {
    // arange
    let mut index = IndexBuilder::new()
        .with_id(0)
        .with_keys_range_to_append(0..100)
        .with_dump_to_file(true)
        .build()
        .await;
    // act
    index.load().await.unwrap();
    // assert
    assert_eq!(index.count(), 100);
}

async fn check_key_push(key: usize, range: Range<usize>, key_map: fn(usize) -> usize) {
    // arange
    let mut index = IndexBuilder::new()
        .with_id(0)
        .with_keys_range_to_append(range)
        .with_key_map(key_map)
        .build()
        .await;
    let rh = RecordHeaderBuilder::new().with_key(key.into()).build();
    // act
    index.push(rh).unwrap();
    // assert
    assert!(index.contains_key(&key.into()).await.unwrap_or(false));
}

#[tokio::test]
async fn check_push_before() {
    check_key_push(1, 11..100, |k| k).await;
}

#[tokio::test]
async fn check_push_after() {
    check_key_push(110, 10..99, |k| k).await;
}

#[tokio::test]
async fn check_push_between() {
    check_key_push(7, 0..10, |k| k * 2).await;
}

#[tokio::test]
async fn check_on_disk_push() {
    // arange
    let mut index = IndexBuilder::new()
        .with_id(0)
        .with_keys_range_to_append(0..10)
        .with_dump_to_file(true)
        .build()
        .await;
    let rh = RecordHeaderBuilder::new().with_key(11.into()).build();
    // act
    let res = index.push(rh);
    // assert
    assert!(res.is_err());
}
