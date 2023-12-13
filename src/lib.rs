pub fn add(a: i32, b: i32) -> i32 {
    let sum = a + b;
    return sum;
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn zero_plus_zero() {
        assert_eq!(0, add(0, 0));
    }

    #[test]
    fn two_plus_two() {
        assert_eq!(4, add(2, 2));
    }

    #[test]
    fn two_plus_zero() {
        assert_eq!(2, add(2, 0));
    }
}
