use std::ops::Deref;


struct MyStackBox<T>(T);

impl<T> MyStackBox<T> {
    fn new(x: T) -> MyStackBox<T> {
        MyStackBox(x)
    }
}

impl<T> Deref for MyStackBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}


fn main() {
    let x = 5;
    let y = MyStackBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyStackBox::new(String::from("Rust"));
    hello(&m);

    println!("Hello, world!");
}
