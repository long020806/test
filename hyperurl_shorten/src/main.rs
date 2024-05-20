use quicli::prelude::*; 
use structopt::StructOpt; 
const CONN_ADDR: &str = "127.0.0.1:3002"; 
/// 这是一个使用hyperurl来缩短网址的小型CLI工具 
/// url 短网址服务 
#[derive(Debug, StructOpt)] 
struct Cli { 
    /// 要缩短的 url 
    #[structopt(long = "url", short = "u")] 
    url: String, 
    // 为该CLI工具配置日志 #[structopt(flatten)] 
    #[structopt(flatten)] 
    verbosity: Verbosity, 
} 
fn main() -> CliResult { 
    let args = Cli::from_args(); 
    println!("Shortening: {}", args.url); 
    let client = reqwest::Client::new(); 
    let mut res = client 
        .post(&format!("http://{}/shorten", CONN_ADDR)) 
        .body(args.url) 
        .send()?; 
    let a: String = res.text().unwrap(); 
    println!("http://{}", a); 
    Ok(()) 
} 