
// what I get here is that if we define our precise data structure we can then implement the Iterator trait for it calling the next() metho and get a way to iterate on our structure

// "The Iterator trait only requires implementors to define one method: the next method, which returns one item of the iterator at a time wrapped in Some and, when iteration is over, returns None."
fn main() {
    let list = vec![1,2,3,4];
    let iterator = list.iter();

    for val in iterator {
        println!("Kuck mal, das habe ich gefunden: {}", val);
    }

    // does not take ownership nor borrow mutably
    let mut iterator2 = list.iter();

    assert_eq!(iterator2.next(), Some(&1));

    // this is another list, which did not modify the original one
    let mut updated_list : Vec<i32> = iterator2.map(|x| x+1 ).collect();

    // in fact here I can use the original again
    println!("{:?} and {:?}", list, updated_list);

    let iterator3 = updated_list.into_iter();

    iterator3.map(|x| x + 1);



    // Note that we needed to make v1_iter mutable: calling the next method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence. In other words, this code consumes, or uses up, the iterator. Each call to next eats up an item from the iterator. We didn’t need to make v1_iter mutable when we used a for loop because the loop took ownership of v1_iter and made it mutable behind the scenes.

    // Also note that the values we get from the calls to next are immutable references to the values in the vector. The iter method produces an iterator over immutable references. If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter. Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.

    // iter_mut -> mutable references.

    // into_iter -> ownership of the values.

}
