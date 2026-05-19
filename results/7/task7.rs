fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let max_a = *a.iter().max().unwrap();
    let min_b = *b.iter().min().unwrap();
    let mut count = 0;
    for x in (max_a..=min_b).step_by(max_a as usize) {
        if a.iter().all(|&ai| x % ai == 0) && b.iter().all(|&bi| bi % x == 0) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x() {
        assert_eq!(get_total_x(&[2, 4], &[16, 32, 96]), 3);
    }
}