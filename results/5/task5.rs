fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = apples.iter().filter(|&&d| a + d >= s && a + d <= t).count();
    let orange_count = oranges.iter().filter(|&&d| b + d >= s && b + d <= t).count();
    println!("{}", apple_count);
    println!("{}", orange_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        count_apples_and_oranges(7, 11, 5, 15, &[2, 3, 4], &[5, -6]);
    }
}