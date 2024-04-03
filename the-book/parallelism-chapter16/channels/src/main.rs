use std::thread;
use std::sync::mpsc;
use std::time::Duration;


fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn( move || {
        let value = String::from("Hi from the other thread!");
        tx.send(value).unwrap();
    });

    println!("Hello, world! In the other world I found this: {}", rx.recv().unwrap() );

    // what if we want to send multiple values?
    // 'Sending Multiple Values and Seeing the Receiver Waiting' + 'Creating Multiple Producers by Cloning the Transmitter'

    let (tx,rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn( move || {
        let words = vec![String::from("what"), String::from("number"), String::from("am I thinking?"), String::from("5")];

        for word in words {
            tx1.send(word).unwrap();
            thread::sleep(Duration::from_millis(700));
        }
    });

    thread::spawn( move || {
        let barbarians = vec![String::from("maybe"), String::from("u"), String::from("dumb?"), String::from("5")];

        for word in barbarians {
            tx.send(word).unwrap();
            thread::sleep(Duration::from_millis(700));
        }
    });

    for received in rx {
        println!("alternative channel: {}", received);
    }

    // note that rx as an iterator does not reach its end till all trnasmitters are closed (dropped)

}
