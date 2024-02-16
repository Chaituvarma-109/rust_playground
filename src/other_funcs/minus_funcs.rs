/// Function: mul_five
///
/// # Arguments (x: u32)
/// 
/// #Returns  u32
/// 
/// 
/// # Examples
/// ```rust
/// let x = 2;
/// let y = mul_five(x);
/// ```
/// 
/**
 * This is a Multiline Block comment
 */

pub fn mul_five(x: u32) -> u32 {
    // multplies the given number i.e., x by 5
    x * 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_five() {
        assert_eq!(10, mul_five(2));
    }

    #[test]
    fn test_mul_zero() {
        assert_eq!(0, mul_five(0));
    }
}
