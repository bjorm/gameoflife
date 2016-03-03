use super::get_valid_indices;

#[test]
fn should_return_correct_neighbours() {
    assert_eq!(get_valid_indices(0, 0, 4), vec![(0, 1), (1, 0), (1, 1)]);
    assert_eq!(get_valid_indices(3, 0, 4), vec![(3, 1), (2, 0), (2, 1)]);   
    assert_eq!(get_valid_indices(0, 3, 4), vec![(0, 2), (1, 3), (1, 2)]);   
    assert_eq!(get_valid_indices(3, 3, 4), vec![(3, 2), (2, 3), (2, 2)]);      
    assert_eq!(get_valid_indices(2, 2, 5), vec![(2, 3), (2, 1), (1, 2), (3, 2), (1, 1), (3, 1), (1, 3), (3, 3)]);      
}
