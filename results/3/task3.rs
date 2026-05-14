pub fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        println!("{}{}", spaces, hashes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase() {
        staircase(4);
        staircase(6);
    }
}