use std::env;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);
impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }
    fn increment(&mut self,word: &str){
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }
    fn display(&self,count:u64) {
        let mut result_vec:Vec<_> = self.0.iter().map(|(key,value)|(key,value)).filter(|(_key,value)|**value>count).collect();
        result_vec.sort_by_key(|item|item.1);
        for (key, value) in result_vec {
            println!("{}: {}", key, value);
        }
    }
}
fn main() {
    let arguments: Vec<String> = env::args().collect();
    // except None的情况抛出异常
    let filename = arguments.get(1).expect("please input filename");
    // unwrap_or None的情况下为or 后内容 类型要和get结果类型保持一致
    let count = arguments.get(2)
            .unwrap_or(&"0".to_string().to_owned())
            .parse::<u64>()
            .unwrap_or(0u64);
    println!("Processing file: {}", filename);
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut word_counter = WordCounter::new();
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");
        for word in words {
            if word == "" {
                continue
            } else {
                word_counter.increment(word);
            }
        }
    }
    word_counter.display(count);
}
