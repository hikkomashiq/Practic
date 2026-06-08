pub fn breaking_records(scores: &[i32]) -> (i32, i32) {
    let mut min_score = scores[0];
    let mut max_score = scores[0];
    let mut min_breaks = 0;
    let mut max_breaks = 0;

    for &score in &scores[1..] {
        if score > max_score {
            max_score = score;
            max_breaks += 1;
        }
        if score < min_score {
            min_score = score;
            min_breaks += 1;
        }
    }

    (max_breaks, min_breaks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records(&scores), (2, 4));
    }

    #[test]
    fn test_sample_input_two() {
        let scores = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        assert_eq!(breaking_records(&scores), (4, 0));
    }

    #[test]
    fn test_single_game() {
        let scores = vec![15];
        assert_eq!(breaking_records(&scores), (0, 0));
    }
}
