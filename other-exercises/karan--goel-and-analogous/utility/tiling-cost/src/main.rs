// Find Cost of Tile to Cover W x H Floor - Calculate the total cost of tile it would take to cover a floor plan of width and height, using a cost entered by the user.

// to complicate it: imagine to be an n dimensional creature, and to have a hierarchy of j dimensional tiles for each j<n, each with a different cost (maybe decreasing/increasing?). Calculate the cost here as well. We assume that the n dimensional creature will want tiles for every dimension j=1..n

use std::io;

fn main() {
    println!("I see that you bought the house for the mortgage of which you were inquiring. This program will help you calculate the cost of certain renovation work.");

    let single_tile_price : f64 = receive_input("Give me please the price of a single tile: ");

    // for now we imagine a tile with no precise shape, but just a certain area which fits perfectly for every usage
    let single_tile_area = receive_input("Give me please the area covered by a single tile: ");
    
    // first a model with living creatures of 3 dimensions
    // let living_dimension : u32 = receive_input("In how many dimension to you live? ") as u32;
    let _living_dimension : u32 = 3;
    let lenght = receive_input("Give me please the length of the room to work on: ");
    let depth = receive_input("Give me please the depth of the room to work on: ");

    let cost = (single_tile_price)*(lenght * depth)/(single_tile_area);

    println!("The cost of renovating your room of length {lenght} and of depth {depth} with tiles of area {single_tile_area} and of price {single_tile_price} each is {cost} measure unit of money.");
}


fn receive_input(message: &str) -> f64 {
    loop {
        println!("{}",message);

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line!");

        let parsed_input : f64 = match user_input.trim().parse() {
            Ok(parsed_input) => if parsed_input > 0.0 {parsed_input} else {continue},
            Err(_) => continue,
        };
        return parsed_input;
    }
}