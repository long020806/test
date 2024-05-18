use std::ops::Add; 
use std::convert::From;
use std::fmt::{Formatter, Display, Result}; 
#[derive(Default, Debug, PartialEq, Copy, Clone)]  
struct Complex<T>{
    // 实部
    re:T,
    // 虚部
    im:T
}

impl<T> Complex<T>{
    fn new(re:T,im:T)->Self{
        Complex { re, im }
    }
}
// pub trait Add<RHS = Self> { 
//     type Output; 
//     fn add(self, rhs: RHS) -> Self::Output; 
// }
// pub trait From<T> { 
//     fn from(self) -> T; 
// } 
// pub trait Display { 
//     fn fmt(&self, &mut Formatter) -> Result<(), Error>; 
// } 
impl<T:Add<T,Output = T>> Add for Complex<T> { 
    type Output = Complex<T>; 
    fn add(self, rhs: Complex<T>) -> Self::Output { 
        // 由于 self.re 内部类型为 T 而此处对 T类型进行了加法 所以 T必须满足可加 
        // 又由于 加和后结果依旧为T 所以T的类型为可加类型 T 且返回类型为T
        Complex { re: self.re + rhs.re, im: self.im + rhs.im } 
    } 
} 

impl<T> From<(T, T)> for Complex<T> {
    fn from(value: (T, T)) -> Complex<T> {
        Complex {
            re: value.0,
            im:value.1
        }
    }
}

impl<T:Display> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter) -> Result { 
        write!(f, "{} + {}i", self.re, self.im) 
    } 
}
#[cfg(test)] 
mod tests { 

    use crate::Complex; 
    #[test] 
    fn complex_basics() { 
        let first = Complex::new(3,5); 
        let second: Complex<i32> = Complex::default(); 
    } 
    #[test] 
    fn complex_addition() { 
        let a = Complex::new(1,-2); 
        let b = Complex::default(); 
        let res = a + b; 
        assert_eq!(res, a); 
    } 

    #[test] 
    fn complex_from() { 
        let a = (2345, 456); 
        let complex = Complex::from(a); 
        assert_eq!(complex.re, 2345); 
        assert_eq!(complex.im, 456); 
    } 


    #[test] 
    fn complex_display() { 
        let my_imaginary = Complex::new(2345,456); 
        println!("{}", my_imaginary); 
    } 
} 