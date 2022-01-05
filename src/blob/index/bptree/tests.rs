use super::prelude::*;

const META_SIZE: usize = 100;
const META_VALUE: u8 = 17;

mod file_mock;
mod findex_builder;
mod findex_mother;
mod generic_scenarios;
mod inmem_builder;
mod key;
mod rh_builder;

use findex_builder::FileIndexBuilder;
use findex_mother::FindexMother;
use generic_scenarios::*;
use inmem_builder::InMemoryIndexBuilder;
use key::KeyType;
use rh_builder::RecordHeaderBuilder;

#[tokio::test]
async fn check_deserialize() {
    check_deserialize_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
    check_deserialize_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_any_first() {
    check_get_any_first_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
    check_get_any_first_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_any_last() {
    check_get_any_last_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
    check_get_any_last_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_any_ordinary() {
    check_get_any_ordinary_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
    check_get_any_ordinary_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_any_absent() {
    check_get_any_absent_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
    check_get_any_absent_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_all_first() {
    check_get_all_first_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
    check_get_all_first_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_all_last() {
    check_get_all_last_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
    check_get_all_last_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_all_ordinary() {
    check_get_all_ordinary_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
    check_get_all_ordinary_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_all_absent() {
    check_get_all_absent_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
    check_get_all_absent_generic::<BPTreeFileIndex<KeyType, File>>().await;
}
