use std::io::stdin;
// 一个方便的宏，用于将输入作为字符串读入缓冲区
macro_rules! scanline {
    ($x:expr) => {{
        stdin().read_line(&mut $x).unwrap();
        $x.trim();
    }};
    () => ({ 
        let mut s = String::new(); 
        stdin().read_line(&mut s).unwrap(); 
        s 
    }); 
}


pub fn main() {
    let mut input = String::new();
    scanline!(input);
    println!("{:?}", input);
}
