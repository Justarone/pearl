use super::*;

#[tokio::test]
async fn check_deserialize_mocked_bptree() {
    check_deserialize_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
}

#[tokio::test]
async fn check_get_any_first_mocked_bptree() {
    check_get_any_first_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
}

#[tokio::test]
async fn check_get_any_last_mocked_bptree() {
    check_get_any_last_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
}

#[tokio::test]
async fn check_get_any_ordinary_mocked_bptree() {
    check_get_any_ordinary_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
}

#[tokio::test]
async fn check_get_any_absent_mocked_bptree() {
    check_get_any_absent_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
}

#[tokio::test]
async fn check_get_all_first_mocked_bptree() {
    check_get_all_first_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
}

#[tokio::test]
async fn check_get_all_last_mocked_bptree() {
    check_get_all_last_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
}

#[tokio::test]
async fn check_get_all_ordinary_mocked_bptree() {
    check_get_all_ordinary_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
}

#[tokio::test]
async fn check_get_all_absent_mocked_bptree() {
    check_get_all_absent_generic::<BPTreeFileIndex<KeyType, file_mock::File>>().await;
}
