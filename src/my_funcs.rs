pub fn add_five(x: u32) -> u32 {
    // Add five to `x`
    x + 5
}

pub fn sub_five(x: u32) -> u32 {
    // Subtract five from `x`
    x - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_five() {
        assert_eq!(10, add_five(5));
    }

    #[test]
    fn test_add_five2() {
        assert_eq!(5, add_five(0));
    }

    #[test]
    fn test_sub_five() {
        assert_eq!(5, sub_five(10));
    }

    #[test]
    fn test_sub_five2() {
        assert_eq!(0, sub_five(5));
    }
}
