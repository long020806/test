use std::{env, process};

use minigrep::Config;


fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("参数解析失败:{}",err);
        process::exit(1);
    });
    // let config = match Config::new(&args) {
    //     Ok(it) => it,
    //     Err(err) => {
    //         return println!("{}",err);
    //     },
    // };

    if let Err(e) = minigrep::run(config){
        eprintln!("应用存在异常:{}",e);
        process::exit(1);
    }

}
