use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn expensive_computation(secs: usize) -> usize {
    thread::sleep(Duration::from_secs(secs as u64));
    secs
}

fn main() {
    let (sender, receiver) = channel();
    let n_sender = 5;
    for i in 0..n_sender {
        let sender_clone = sender.clone();
        thread::spawn(move || {
            sender_clone.send(expensive_computation(i)).unwrap();
        });
    }

    println!("doing work here");
    thread::sleep(Duration::from_secs(3));
    println!("main thread done");

    /*
    for _ in 0..n_sender {
        println!("{:?}", receiver.recv().unwrap());
    }
    */
    loop {
        println!("{:?}", receiver.recv().unwrap());
    }

}
