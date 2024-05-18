fn main() { 
    let mut a = String::from("Owned string"); 
    let a_ref = &mut a; 
    let b = & a;// Error 有了可变引用不能有不可变引用 （自身除外）
    a_ref.push('!'); 
    println!("{}", a); 
} 