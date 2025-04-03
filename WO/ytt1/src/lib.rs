/* How to write tests in rust

Tests in rust are written within the same fiel as the code they are testing.
The bodies oftest sare marked with the #[test] attribute,
and they perform three main tasks:

1. Setup any needed data or state
2. Run the code you want to test
3. Asset the results are what you expect


*/

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
