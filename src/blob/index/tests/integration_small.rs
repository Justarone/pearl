use super::*;
// with 2 classes: BPTreeFileIndex + File

#[tokio::test]
async fn check_deserialize() {
    check_deserialize_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_any_first() {
    check_get_any_first_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_any_last() {
    check_get_any_last_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_any_ordinary() {
    check_get_any_ordinary_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_any_absent() {
    check_get_any_absent_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_all_first() {
    check_get_all_first_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_all_last() {
    check_get_all_last_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_all_ordinary() {
    check_get_all_ordinary_generic::<BPTreeFileIndex<KeyType, File>>().await;
}

#[tokio::test]
async fn check_get_all_absent() {
    check_get_all_absent_generic::<BPTreeFileIndex<KeyType, File>>().await;
}
