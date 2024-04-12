use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    thread::spawn(|| {
        for i in 1..10 {
            println!("parallel thread {i}");
            thread::sleep(Duration::from_millis(1));
        }
    } );

    for i in 1..10 {
        println!("number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // not interesting if we cannot wait for the threads to finish
    // We can fix the problem of the spawned thread not running or ending prematurely by saving the return value of thread::spawn in a variable. The return type of thread::spawn is JoinHandle. A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish.

    let handle = thread::spawn( || {
        for num in 1..49 {
            println!("Incredible thread: number {}", num);
        }
    });

    for i in 1..10 {
        println!("main thread: number {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // I guess I might interpret 'join' as a 'join with the main thread'
    handle.join().unwrap();

}
