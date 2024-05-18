pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn sum(a: i8, b: i8) -> i8 { 
    a + b 
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
