fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();

    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return 0;
    }

    a.abs() / gcd(a, b) * b.abs()
}

pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let lcm_a = a
        .iter()
        .map(|&value| value as i64)
        .reduce(|acc, value| lcm(acc, value))
        .unwrap_or(1);

    let gcd_b = b
        .iter()
        .map(|&value| value as i64)
        .reduce(|acc, value| gcd(acc, value))
        .unwrap_or(0);

    if lcm_a == 0 || gcd_b == 0 || lcm_a > gcd_b {
        return 0;
    }

    let mut count = 0;
    let mut x = lcm_a;

    while x <= gcd_b {
        if gcd_b % x == 0 {
            count += 1;
        }
        x += lcm_a;
    }

    count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let a = vec![2, 6];
        let b = vec![24, 36];
        assert_eq!(get_total_x(&a, &b), 2);
    }

    #[test]
    fn test_sample_input_two() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }

    #[test]
    fn test_no_valid_numbers() {
        let a = vec![3, 9];
        let b = vec![10, 14];
        assert_eq!(get_total_x(&a, &b), 0);
    }
}
