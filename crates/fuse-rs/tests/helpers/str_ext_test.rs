use fuse_rs::helpers::str_ext::StrExt;

#[test]
fn test_strip_diacritics_basic() {
    let input = "café naïve résumé";
    let expected = "cafe naive resume";
    assert_eq!(input.strip_diacritics(), expected);
}

#[test]
fn test_strip_diacritics_mixed() {
    let input = "Crème brûlée – déjà vu";
    let expected = "Creme brulee – deja vu";
    assert_eq!(input.strip_diacritics(), expected);
}

#[test]
fn test_strip_diacritics_no_diacritics() {
    let input = "hello world";
    let expected = "hello world";
    assert_eq!(input.strip_diacritics(), expected);
}

#[test]
fn test_strip_diacritics_empty() {
    let input = "";
    let expected = "";
    assert_eq!(input.strip_diacritics(), expected);
}

#[test]
fn test_index_of() {
    let input = "hello world".to_string();
    assert_eq!(input.index_of("world", Some(0)), Some(6));
    assert_eq!(input.index_of("hello", Some(0)), Some(0));
    assert_eq!(input.index_of("o", Some(4)), Some(4));
    assert_eq!(input.index_of("x", Some(0)), None);
    assert_eq!(input.index_of("world", Some(7)), None);
}