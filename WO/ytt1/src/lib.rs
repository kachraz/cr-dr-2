/* How to write tests in rust

Tests in rust are written within the same fiel as the code they are testing.
The bodies oftest sare marked with the #[test] attribute,
and they perform three main tasks:

1. Setup any needed data or state
2. Run the code you want to test
3. Asset the results are what you expect

*/

// Anatomy of test function
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn multiply(left: u64, right: u64) -> u64 {
    left * right
}
pub fn divide(left: u64, right: u64) -> u64 {
    left / right
}
pub fn subtract(left: u64, right: u64) -> u64 {
    left - right
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works_fuck() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn it_works_multiply() {
//         let result = multiply(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn it_works_divide() {
//         let result = divide(4, 2);
//         assert_eq!(result, 2);
//     }

//     #[test]
//     fn it_wil_fail() {
//         panic!("😭 Failed");
//     }
// }

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    struct Rectangle {
        width: u32
        height: u32
    }
    
}
