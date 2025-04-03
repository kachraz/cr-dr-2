/* How to write tests in rust

Tests in rust are written within the same fiel as the code they are testing.
The bodies oftest sare marked with the #[test] attribute,
and they perform three main tasks:

1. Setup any needed data or state
2. Run the code you want to test
3. Asset the results are what you expect

*/

// Adding a custom failure message

pub fn greeting(name: &str) -> String {
    // format!("Hello, {}!", name)
    String::from("Hello, ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting() {
        let name = "Alice";
        let expected = "Hello, Alice!";
        let result = greeting(name);
        assert_eq!(
            result, expected,
            "One Expected greeting to be '{}', but got '{}'",
            expected, result
        );
    }

    #[test]
    fn test_greeting_empty() {
        let name = "";
        let expected = "Hello, !";
        let result = greeting(name);
        assert_eq!(
            result, expected,
            "TWO ❌ Expected greeting to be '{}', but got '{}'",
            expected, result
        );
    }
}
