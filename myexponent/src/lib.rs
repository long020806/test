fn pow(base: i64, exponent: usize) -> i64 { 
    let mut res = 1;
    if exponent == 1 {
        return 1;
    }
    for _ in 0..exponent {
        res = res * base;
    }
    res
} 
#[cfg(test)] 
mod tests { 
    use super::pow; 
    #[test] 
    fn minus_two_raised_three_is_minus_eight() { 
        assert_eq!(pow(-2, 3), -8); 
    } 
} 
