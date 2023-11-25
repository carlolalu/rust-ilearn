fn main() {

    let mut life_is_a_lie = true;

    let my_array : [i16;28] = [3;28];

    for _element in my_array {
        while life_is_a_lie {
            println!("cry as a baby");
            if !is_life_a_lie(life_is_a_lie) {
                life_is_a_lie = false;
            }
        }
    }



    println!("Hello, world!");
}


fn is_life_a_lie(option : bool) -> bool {
    !option
}