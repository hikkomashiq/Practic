#[allow(dead_code)]
pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades
        .iter()
        .map(|&grade| {
            if grade < 38 {
                grade
            } else {
                let next_multiple = ((grade / 5) + 1) * 5;

                if next_multiple - grade < 3 {
                    next_multiple
                } else {
                    grade
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let grades = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(&grades), expected);
    }

    #[test]
    fn test_rounding_boundary() {
        let grades = vec![84, 85, 57, 58, 37, 38];
        let expected = vec![85, 85, 57, 60, 37, 40];
        assert_eq!(grading_students(&grades), expected);
    }

    #[test]
    fn test_failing_grades_unchanged() {
        let grades = vec![0, 37, 33];
        let expected = vec![0, 37, 33];
        assert_eq!(grading_students(&grades), expected);
    }
}
