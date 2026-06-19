use heck::ToSnakeCase;

/// Converts a string slice to snake_case.
pub fn to_snake(input: &str) -> String {
    input.to_snake_case()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake() {
        assert_eq!(to_snake("Demo Cargo Pkg"), "demo_cargo_pkg");
    }
}
