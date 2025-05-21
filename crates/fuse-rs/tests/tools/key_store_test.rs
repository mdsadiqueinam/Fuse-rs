use fuse_rs::tools::key_store::KeyStore;
use fuse_rs::core::options::keys::{FuseOptionKey, FuseOptionKeyName, FuseOptionKeyObject};
use std::borrow::Cow;

const EPSILON: f64 = 1e-10;

#[test]
fn test_key_store_creation() {
    let keys = vec![
        FuseOptionKey::String(Cow::Borrowed("name")),
        FuseOptionKey::StringArray(vec![Cow::Borrowed("author"), Cow::Borrowed("name")]),
        FuseOptionKey::KeyObject(FuseOptionKeyObject {
            name: Cow::Borrowed(&FuseOptionKeyName::String(Cow::Borrowed("title"))),
            weight: Some(2.0),
            get_fn: None,
        }),
    ];

    let key_store = KeyStore::new(&keys);

    assert_eq!(key_store.keys().len(), 3);

    let total_weight: f64 = key_store.keys().iter().map(|k| k.weight).sum();
    assert!((total_weight - 1.0).abs() < EPSILON);

    let title_key = key_store.get("title").unwrap();
    assert_eq!(title_key.src, "title");
    assert!(title_key.weight > 0.0);
}