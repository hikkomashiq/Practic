pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> &'static str {
    if v1 == v2 {
        return if x1 == x2 { "YES" } else { "NO" };
    }

    let diff = x2 - x1;
    let rate = v1 - v2;

    if diff % rate == 0 && diff / rate >= 0 {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_yes() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_sample_no() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_same_speed_different_position() {
        assert_eq!(kangaroo(43, 2, 70, 2), "NO");
    }

    #[test]
    fn test_same_speed_same_position() {
        assert_eq!(kangaroo(5, 3, 5, 3), "YES");
    }
}
