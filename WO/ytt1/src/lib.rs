/* How to write tests in rust

Tests in rust are written within the same fiel as the code they are testing.
The bodies oftest sare marked with the #[test] attribute,
and they perform three main tasks:

1. Setup any needed data or state
2. Run the code you want to test
3. Asset the results are what you expect

*/

// Checking the panic atttribute

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
