trait Vegetable {
    fn function(&self) {
        todo!();
    }
    
}

struct MonotonousSalad<V : Vegetable> {
    ingredients : Vec<V>,
}

struct Salad {
    ingredients : Vec< Box<dyn Vegetable> >,
}


fn main() {
    println!("Hello, world!");
}
