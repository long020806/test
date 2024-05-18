use std::{thread, time::Duration, vec, sync::mpsc};



fn main(){

    let v = vec![1,2,3];
    // let handle = thread::spawn(||{
    //     for i in 1..10{
    //         println!("number {}",i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // handle.join().unwrap();

    // for i in 1..5{
    //     println!("main number {}",i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    let handle = thread::spawn(move ||{
        println!("number {:?}",v);
    });


}