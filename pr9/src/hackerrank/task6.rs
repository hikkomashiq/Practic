pub fn sock_merchant(ar: &[i32]) -> i32 {
    let mut color_counts = std::collections::HashMap::new();

    for &color in ar {
        *color_counts.entry(color).or_insert(0) += 1;
    }

    color_counts.values().map(|count| count / 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(&ar), 3);
    }

    #[test]
    fn test_counts_multiple_colors() {
        let ar = vec![1, 1, 1, 1, 2, 2, 3];
        assert_eq!(sock_merchant(&ar), 3);
    }

    #[test]
    fn test_no_pairs() {
        let ar = vec![1, 2, 3, 4];
        assert_eq!(sock_merchant(&ar), 0);
    }
}
