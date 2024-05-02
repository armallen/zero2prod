fn main() {
    println!("Hello, world!");
}

// Add a test for main function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
