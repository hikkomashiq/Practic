pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0i32; 6];

    for &bird in arr {
        counts[bird as usize] += 1;
    }

    let mut max_freq = 0;
    let mut result = 1;

    for bird_type in 1..=5 {
        if counts[bird_type] > max_freq {
            max_freq = counts[bird_type];
            result = bird_type as i32;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_sample_input_two() {
        let arr = vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];
        assert_eq!(migratory_birds(&arr), 3);
    }

    #[test]
    fn test_tie_returns_smallest_id() {
        let arr = vec![1, 1, 2, 2, 3];
        assert_eq!(migratory_birds(&arr), 1);
    }
}
