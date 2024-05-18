enum Level { 
    Error 
} 
struct Logger<'a>(&'a str, Level); 
fn configure_logger<T>(_t: T) where T: Send + 'static { 
    // 
这里配置logger 
} 

fn main() { 
    let other = String::from("Local"); 
    let log2 = Logger(&other, Level::Error); 
    configure_logger(&log2); 
}