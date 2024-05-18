use std::{thread, sync::mpsc, time::Duration};

fn main(){
    let (tx,rx) = mpsc::channel();

    thread::spawn(move ||{
        let vals = vec![String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread")
        ];
        for ele in vals {
            tx.send(ele).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for ele in rx {
        println!("Got :{}",ele);
    }


}