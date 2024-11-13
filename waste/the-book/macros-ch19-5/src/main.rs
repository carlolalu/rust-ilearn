// Let’s create a crate named hello_macro that defines a trait named HelloMacro with one associated function named hello_macro. Rather than making our users implement the HelloMacro trait for each of their types, we’ll provide a procedural macro so users can annotate their type with #[derive(HelloMacro)] to get a default implementation of the hello_macro function. The default implementation will print Hello, Macro! My name is TypeName! where TypeName is the name of the type on which this trait has been defined. Let’s create a crate named hello_macro that defines a trait named HelloMacro with one associated function named hello_macro. Rather than making our users implement the HelloMacro trait for each of their types, we’ll provide a procedural macro so users can annotate their type with #[derive(HelloMacro)] to get a default implementation of the hello_macro function. The default implementation will print Hello, Macro! My name is TypeName! where TypeName is the name of the type on which this trait has been defined.



fn main() {
    println!("Hello, world!");
}
