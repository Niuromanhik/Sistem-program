fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&g| {
        if g < 38 {
            return g;
        }
        let next_multiple = (g / 5 + 1) * 5;
        if next_multiple - g < 3 {
            next_multiple
        } else {
            g
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading() {
        assert_eq!(grading_students(&[73, 67, 38, 33]), vec![75, 67, 40, 33]);
    }
}