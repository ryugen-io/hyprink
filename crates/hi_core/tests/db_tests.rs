use hi_core::db::Store;
use tempfile::tempdir;

mod common;

#[test]
fn test_store_new_empty() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("store.bin");
    let store = Store::load(&path).unwrap();
    assert!(store.list().is_empty());
}

#[test]
fn test_store_add_template() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("store.bin");
    let mut store = Store::load(&path).unwrap();
    let tpl = common::create_test_template("test-tpl");
    store.add(tpl).unwrap();
    assert_eq!(store.list().len(), 1);
    assert_eq!(store.list()[0].manifest.name, "test-tpl");
}

#[test]
fn test_store_remove_template() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("store.bin");
    let mut store = Store::load(&path).unwrap();
    store
        .add(common::create_test_template("to-remove"))
        .unwrap();
    let removed = store.remove("to-remove");
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().manifest.name, "to-remove");
    assert!(store.list().is_empty());
}

#[test]
fn test_store_remove_nonexistent() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("store.bin");
    let mut store = Store::load(&path).unwrap();
    let removed = store.remove("does-not-exist");
    assert!(removed.is_none());
}

#[test]
fn test_store_clear() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("store.bin");
    let mut store = Store::load(&path).unwrap();
    store.add(common::create_test_template("tpl1")).unwrap();
    store.add(common::create_test_template("tpl2")).unwrap();
    store.add(common::create_test_template("tpl3")).unwrap();
    assert_eq!(store.list().len(), 3);
    store.clear();
    assert!(store.list().is_empty());
}

#[test]
fn test_store_set_ignored() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("store.bin");
    let mut store = Store::load(&path).unwrap();
    store
        .add(common::create_test_template("ignorable"))
        .unwrap();
    assert!(!store.list()[0].manifest.ignored);
    let result = store.set_ignored("ignorable", true).unwrap();
    assert!(result);
    assert!(store.list()[0].manifest.ignored);
    let result = store.set_ignored("ignorable", false).unwrap();
    assert!(result);
    assert!(!store.list()[0].manifest.ignored);
}

#[test]
fn test_store_set_ignored_nonexistent() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("store.bin");
    let mut store = Store::load(&path).unwrap();
    let result = store.set_ignored("does-not-exist", true).unwrap();
    assert!(!result);
}

#[test]
fn test_store_list_sorted() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("store.bin");
    let mut store = Store::load(&path).unwrap();
    store.add(common::create_test_template("zebra")).unwrap();
    store.add(common::create_test_template("alpha")).unwrap();
    store.add(common::create_test_template("middle")).unwrap();
    let list = store.list();
    assert_eq!(list[0].manifest.name, "alpha");
    assert_eq!(list[1].manifest.name, "middle");
    assert_eq!(list[2].manifest.name, "zebra");
}

#[test]
fn test_store_save_and_load() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("store.bin");
    {
        let mut store = Store::load(&path).unwrap();
        store
            .add(common::create_test_template("persisted"))
            .unwrap();
        store.save().unwrap();
    }
    let store = Store::load(&path).unwrap();
    assert_eq!(store.list().len(), 1);
    assert_eq!(store.list()[0].manifest.name, "persisted");
}

#[test]
fn test_store_save_creates_parent_dirs() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("nested").join("dirs").join("store.bin");
    let mut store = Store::load(&path).unwrap();
    store
        .add(common::create_test_template("nested-test"))
        .unwrap();
    store.save().unwrap();
    assert!(path.exists());
}

#[test]
fn test_store_iter() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("store.bin");
    let mut store = Store::load(&path).unwrap();
    store.add(common::create_test_template("iter1")).unwrap();
    store.add(common::create_test_template("iter2")).unwrap();
    let count = store.iter().count();
    assert_eq!(count, 2);
}

#[test]
fn test_store_overwrite_template() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("store.bin");
    let mut store = Store::load(&path).unwrap();
    let mut tpl1 = common::create_test_template("same-name");
    tpl1.manifest.version = "1.0.0".to_string();
    store.add(tpl1).unwrap();
    let mut tpl2 = common::create_test_template("same-name");
    tpl2.manifest.version = "2.0.0".to_string();
    store.add(tpl2).unwrap();
    assert_eq!(store.list().len(), 1);
    assert_eq!(store.list()[0].manifest.version, "2.0.0");
}
