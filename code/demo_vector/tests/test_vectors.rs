// Integration tests
use demo_vector::compute_sum;

#[test]
fn compute_sum_of_vector() {
    let vec = vec![1, 5, 10, 2, 15];
    let sum = compute_sum(&vec);
    assert_eq!(sum, 33);
}
