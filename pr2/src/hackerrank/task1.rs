pub fn staircase(n: usize) -> Vec<String> {
    let mut result = Vec::new();

    for i in 1..=n {
        result.push(format!(
            "{}{}",
            " ".repeat(n - i),
            "#".repeat(i)
        ));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn staircase_height_4() {
        let expected = vec![
            "   #".to_string(),
            "  ##".to_string(),
            " ###".to_string(),
            "####".to_string(),
        ];

        assert_eq!(staircase(4), expected);
    }
}