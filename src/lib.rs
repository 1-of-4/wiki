pub mod util {
    pub fn parse_args(args: Vec<String>) -> String {
        return args[1..].join(" ");
    }
}

#[cfg(test)]
mod tests {
    use super::util::parse_args;
    #[test]
    fn test_parse() {
        let test_out = String::from("some arguments");
        let args: Vec<String> = "have some arguments".split_whitespace().map(|s| s.to_string()).collect();
        let parsed = parse_args(args);
        assert_eq!(test_out, parsed);
    }
}