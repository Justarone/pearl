use super::*;

pub(super) async fn check_deserialize_generic<FIndex: FileIndexTrait<KeyType>>() {
    // arange
    let inmem = InMemoryIndexBuilder::new()
        .with_from(0)
        .with_to(100)
        .with_index_to_key_fn(|i| (i * 100).into())
        .build();
    let findex: FIndex = FileIndexBuilder::new()
        .with_path(PathBuf::from("/tmp/file_index_deserialize.test"))
        .with_recreate_index_file(true)
        .with_inmem(&inmem)
        .build()
        .await;
    // act
    let (inmem_after, _size) = findex
        .get_records_headers()
        .await
        .expect("Can't get InMemoryIndex");
    // assert
    assert_eq!(inmem, inmem_after);
}

pub(super) async fn check_get_any_first_generic<FIndex: FileIndexTrait<KeyType>>() {
    // arange
    let file = "/tmp/check_get_any_first_generic.test";
    let findex = FindexMother::create_with_step_and_offset::<FIndex, 31, 100>(100, file).await;
    let key: KeyType = 31.into();
    let exp_res = RecordHeaderBuilder::new().with_key(key.clone()).build();
    // act
    let res = findex.get_any(&key).await.unwrap();
    // assert
    assert_eq!(res, Some(exp_res));
}

pub(super) async fn check_get_any_last_generic<FIndex: FileIndexTrait<KeyType>>() {
    // arange
    let file = "/tmp/check_get_any_last_generic.test";
    let findex = FindexMother::create_with_step_and_offset::<FIndex, 31, 100>(100, file).await;
    let key: KeyType = 9931.into(); // last = 31 + 99 * 100
    let exp_res = RecordHeaderBuilder::new().with_key(key.clone()).build();
    // act
    let res = findex.get_any(&key).await.unwrap();
    // assert
    assert_eq!(res, Some(exp_res));
}

pub(super) async fn check_get_any_ordinary_generic<FIndex: FileIndexTrait<KeyType>>() {
    // arange
    let file = "/tmp/check_get_any_ordinary_generic.test";
    let findex = FindexMother::create_with_step_and_offset::<FIndex, 31, 100>(100, file).await;
    let x = 12;
    let key: KeyType = (31 + x * 100).into(); // x-th = 31 + x * 100
    let exp_res = RecordHeaderBuilder::new().with_key(key.clone()).build();
    // act
    let res = findex.get_any(&key).await.unwrap();
    // assert
    assert_eq!(res, Some(exp_res));
}

pub(super) async fn check_get_any_absent_generic<FIndex: FileIndexTrait<KeyType>>() {
    // arange
    let file = "/tmp/check_get_any_absent_generic.test";
    let findex = FindexMother::create_with_step_and_offset::<FIndex, 31, 100>(100, file).await;
    let x = 12;
    let key: KeyType = (31 + x * 100 + 1).into();
    // act
    let res = findex.get_any(&key).await.unwrap();
    // assert
    assert_eq!(res, None);
}

pub(super) async fn check_get_all_first_generic<FIndex: FileIndexTrait<KeyType>>() {
    // arange
    let file = "/tmp/check_get_all_first_generic.test";
    let findex = FindexMother::create_with_reps::<FIndex, 7>(100, file).await;
    let key: KeyType = 0.into();
    let exp_res_elem = RecordHeaderBuilder::new().with_key(key.clone()).build();
    let exp_res: Vec<_> = (0..7).map(|_| exp_res_elem.clone()).collect();
    // act
    let res = findex.find_by_key(&key).await.unwrap();
    // assert
    assert_eq!(res, Some(exp_res));
}

pub(super) async fn check_get_all_last_generic<FIndex: FileIndexTrait<KeyType>>() {
    // arange
    let file = "/tmp/check_get_all_last_generic.test";
    let findex = FindexMother::create_with_reps::<FIndex, 7>(100, file).await;
    let key: KeyType = 99.into();
    let exp_res_elem = RecordHeaderBuilder::new().with_key(key.clone()).build();
    let exp_res: Vec<_> = (0..7).map(|_| exp_res_elem.clone()).collect();
    // act
    let res = findex.find_by_key(&key).await.unwrap();
    // assert
    assert_eq!(res, Some(exp_res));
}

pub(super) async fn check_get_all_ordinary_generic<FIndex: FileIndexTrait<KeyType>>() {
    // arange
    let file = "/tmp/check_get_all_ordinary_generic.test";
    let findex = FindexMother::create_with_reps::<FIndex, 7>(100, file).await;
    let key: KeyType = 7.into();
    let exp_res_elem = RecordHeaderBuilder::new().with_key(key.clone()).build();
    let exp_res: Vec<_> = (0..7).map(|_| exp_res_elem.clone()).collect();
    // act
    let res = findex.find_by_key(&key).await.unwrap();
    // assert
    assert_eq!(res, Some(exp_res));
}

pub(super) async fn check_get_all_absent_generic<FIndex: FileIndexTrait<KeyType>>() {
    // arange
    let file = "/tmp/check_get_all_absent_generic.test";
    let findex = FindexMother::create_with_reps::<FIndex, 7>(100, file).await;
    let key: KeyType = 100.into();
    // act
    let res = findex.find_by_key(&key).await.unwrap();
    // assert
    assert_eq!(res, None);
}
