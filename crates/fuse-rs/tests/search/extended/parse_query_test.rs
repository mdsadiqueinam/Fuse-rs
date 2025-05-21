// Tests for parse_query
use fuse_rs::search::extended::parse_query::parse_query;
use fuse_rs::FuseOptions;

#[test]
fn test_parse_query_simple() {
    let options = FuseOptions::default();
    let pattern = "^core go$ | rb$ | py$ xy$";
    let parsed = parse_query(pattern, &options);
    // Should parse into 3 OR groups
    assert_eq!(parsed.len(), 3);
    // First group: ["^core", "go$"]
    assert_eq!(parsed[0].len(), 2);
    // Second group: ["rb$"]
    assert_eq!(parsed[1].len(), 1);
    // Third group: ["py$", "xy$"]
    assert_eq!(parsed[2].len(), 2);
}
