pub fn binary_search_vector<T: Ord>(vec: &Vec<T>, target: T) -> Option<usize> {
    let mut left = 0;
    let mut right = vec.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if vec[mid] == target {
            return Some(mid);
        } else if vec[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

pub fn compute_sum<T: Ord>(vec: &Vec<T>) -> T {
    let mut sum = 0;
    for i in vec {
        sum += i;
    }
    sum
}
