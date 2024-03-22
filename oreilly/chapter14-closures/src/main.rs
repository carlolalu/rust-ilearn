// the way I should think aboout closure is as STRUCTS which implement some functions and capture the nominated valuesi ntheir environment. Beside, they implement the FnSomthing triats depending on how they capture the values. Fn traits have simliar type to functions:
// fn(type1, type2) -> type3
// FnSomething(t1, t2) -> t3

// here we do our own router as in the section 'callbacks' of chapter 14    

use std::collections::HashMap;


struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

type BoxedCallback = Box <dyn Fn(&Request) -> Response>;

struct BasicRouter {
    routes: HashMap<String, BoxedCallback>,
}

impl BasicRouter {
    fn new() -> BasicRouter {
        BasicRouter{ routes: HashMap::new() }
    }

// Note the two bounds on C in the type signature for add_route: a
// particular Fn trait and the 'static lifetime. Rust makes us add this
// 'static bound. Without it, the call to Box::new(callback) would
// be an error, because it’s not safe to store a closure if it contains bor‐
// rowed references to variables that are about to go out of scope.

// think about closures as structs, to understand this requirement

    fn add_route<C>(&mut self, url: &str, callback: C) 
        where C : Fn(&Request) -> Response + 'static{
            self.routes.insert(url.to_string(), Box::new(callback));
        }

    fn not_found_response() -> Response {
        todo!();
    }

    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => Self::not_found_response(),
            Some(callback) => callback(request)
        }
    }
}

// Now the job of a router is simply to store a table that maps URLs to callbacks so that the right callback can be called on demand. (For simplicity’s sake, we’ll only allow users to create routes that match a single exact URL.)

fn main() {
    println!("Hello, world!");
}
