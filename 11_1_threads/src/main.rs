use std::thread;
use std::time::Duration;


fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let v = vec![1, 2, 3];
    // use move to give control of v to closure
    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    for i in 1..5 {
        println!("number {} from main!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //wait till the job in spawned thread is finished
    handle.join().unwrap();
    handle2.join().unwrap();
}
