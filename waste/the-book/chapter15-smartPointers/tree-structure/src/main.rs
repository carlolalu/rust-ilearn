use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
struct Node {
    value: i32,
    // node are not modifiable once created, but it is modifiable their paternity
    children: RefCell<Vec<Rc<Node>>>,
    // how to get conscience to a leaf, that it has a parent node? (ofc without risking memory leaks)
    // frage: why do we implement this with a refcell? We do not need to modify this.
    // we could do this with an Rc which own the Weak, which does NOT own the parent
    parent: RefCell<Weak<Node>>,
}


fn main() {
    let leaf = Rc::new(Node{
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value:5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!("branch = {:?}", branch);
}
