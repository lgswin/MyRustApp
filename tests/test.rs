#[cfg(test)]
mod tests {
    use my_rust_app::greet;

    #[test]
    fn test_greet() {
        let greeting = greet("Alice");
        assert_eq!(greeting.message, "Hello, Alice!");
    }

    #[test]
    fn test_greet_empty() {
        let greeting = greet("");
        assert_eq!(greeting.message, "Hello, !");
    }

    #[test]
    fn test_greet_special_chars() {
        let greeting = greet("@User123");
        assert_eq!(greeting.message, "Hello, @User123!");
    }
}