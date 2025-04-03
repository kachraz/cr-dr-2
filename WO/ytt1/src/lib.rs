/* How to write tests in rust

Tests in rust are written within the same fiel as the code they are testing.
The bodies oftest sare marked with the #[test] attribute,
and they perform three main tasks:

1. Setup any needed data or state
2. Run the code you want to test
3. Asset the results are what you expect

*/

// Testing Equalaity with assert_eq! and assert_ne!

pub fn add(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add_not_equal() {
        let result = add(2);
        assert_ne!(result, 5);
    }
}
