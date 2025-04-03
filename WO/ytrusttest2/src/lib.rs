fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prints_and_returns_10() {
        let result = prints_and_returns_10(5);
        assert_eq!(result, 10);
    }
}
