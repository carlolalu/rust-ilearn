use std::thread;

fn main() {

    // let us see what does a closure do

    let mut list = vec![1,2,3,4];

    // the closures are anonymous functions, they have thus arguments (between "|"), but most of all they can CAPTURE the environment, meaning that they can see and access the objects in the scope in which they are defined. This means that they can access such variable in the three possible ways: immutable reference, mutable reference and move ownership

    // the standard is to take an immutable reference. To take a mutable one we must have that the var in which is stored the closure is mutable. (the syntax used to captured values is the same as non-reference obj, even though they are references).

    // to move ownership we must force the move by calling 'move' before the declaration of the argument (and this will influence all the values CAPTURED by teh closure), otherwise the values will be only borrowed.

    // if the valued are captured with references (even though we use them with the syntax of vars), they are considered so from the declaration of the closure to its last usage.

    let only_borrow = || println!("{:?}", list);
    // in this space the value is considered borrowed by only_borrow
    only_borrow();

    let mut borrows_mutably = || list.push(7);
    borrows_mutably();

    // how to return the moved value from the thread?
    thread::spawn(move || {
        println!("{:?}", list);
    }).join().unwrap();


    // quote: "A closure body can do any of the following: move a captured value out of the closure, mutate the captured value, neither move nor mutate the value, or capture nothing from the environment to begin with."

    println!("Hello, world!");
}
