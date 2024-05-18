enum Level { 
    Error 
} 
struct Logger<'a>(&'a str, Level); 
fn configure_logger<T>(_t: T) where T: Send + 'static { 
    // 
此处配置logger 
} 
fn main() { 
    let name = "Global"; 
    let log1 = Logger(name, Level::Error); 
    configure_logger(log1); 
}