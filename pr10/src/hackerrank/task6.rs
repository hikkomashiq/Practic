pub fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_sum = 0;
    let mut secondary_sum = 0;

    for i in 0..n {
        primary_sum += arr[i][i];
        secondary_sum += arr[i][n - 1 - i];
    }

    (primary_sum - secondary_sum).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let arr = vec![vec![11, 2, 4], vec![4, 5, 6], vec![10, 8, -12]];
        assert_eq!(diagonal_difference(&arr), 15);
    }

    #[test]
    fn test_equal_diagonal_sums() {
        let arr = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(diagonal_difference(&arr), 0);
    }

    #[test]
    fn test_single_element_matrix() {
        let arr = vec![vec![7]];
        assert_eq!(diagonal_difference(&arr), 0);
    }
}
