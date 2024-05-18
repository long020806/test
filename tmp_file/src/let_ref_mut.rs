#[derive(Debug)] 
struct Items(u32); 
pub fn main() { 
    let items = Items(2);
    let items_ptr = &items; 
    let ref items_ref = items; 
    assert_eq!(items_ptr as *const Items, items_ref as *const Items); 
    let mut a = Items(20); 
    // 通过作用域将b对a的改动限制在内部代码块中 
                // 也可以像这样使用可变引用 
                let ref mut b = a; // same as: let b = &mut a; 
                b.0 += 25; 

    println!("{:?}", items); 
    println!("{:?}", a);      // 没有上述作用域的限制，代码将无法通过编译 
                              // 尝试将上述作用域删除，看看结果如何？ 
} 