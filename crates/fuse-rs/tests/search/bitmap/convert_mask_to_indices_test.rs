use fuse_rs::search::bitmap::convert_mask_to_indices::convert_mask_to_indices;

#[test]
fn test_convert_mask_to_indices() {
    // Test case with multiple matches
    let mask = vec![false, true, true, true, false, false, true, true, true, true, false];
    let result = convert_mask_to_indices(&mask, 3);
    assert_eq!(result, vec![(1, 3), (6, 9)]);

    // Test case with match exactly at minimum length
    let mask = vec![true, true, false];
    let result = convert_mask_to_indices(&mask, 2);
    assert_eq!(result, vec![(0, 1)]);

    // Test case with match below minimum length
    let mask = vec![true, false, true];
    let result = convert_mask_to_indices(&mask, 2);
    assert_eq!(result, Vec::<(usize, usize)>::new());

    // Test case with match at end of array
    let mask = vec![false, false, true, true, true];
    let result = convert_mask_to_indices(&mask, 3);
    assert_eq!(result, vec![(2, 4)]);

    // Test case with empty array
    let mask: Vec<bool> = vec![];
    let result = convert_mask_to_indices(&mask, 1);
    assert_eq!(result, Vec::<(usize, usize)>::new());
}