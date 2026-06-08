pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (usize, usize) {
    let apple_count = apples
        .iter()
        .filter(|&&d| {
            let pos = a + d;
            pos >= s && pos <= t
        })
        .count();

    let orange_count = oranges
        .iter()
        .filter(|&&d| {
            let pos = b + d;
            pos >= s && pos <= t
        })
        .count();

    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];
        let (apple_count, orange_count) = count_apples_and_oranges(7, 11, 5, 15, &apples, &oranges);
        assert_eq!(apple_count, 1);
        assert_eq!(orange_count, 1);
    }

    #[test]
    fn test_house_boundaries() {
        let apples = vec![2, 6];
        let oranges = vec![-4];
        let (apple_count, orange_count) = count_apples_and_oranges(7, 11, 5, 15, &apples, &oranges);
        assert_eq!(apple_count, 2);
        assert_eq!(orange_count, 1);
    }

    #[test]
    fn test_no_fruits_on_house() {
        let apples = vec![-10, 100];
        let oranges = vec![50, -50];
        let (apple_count, orange_count) = count_apples_and_oranges(7, 11, 5, 15, &apples, &oranges);
        assert_eq!(apple_count, 0);
        assert_eq!(orange_count, 0);
    }
}
