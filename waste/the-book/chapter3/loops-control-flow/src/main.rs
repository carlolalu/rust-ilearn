fn main() {

    let mut counter_1 = 0;
    let mut counter_2 = 0;
    let mut counter_3 = 0;

    let var = 'label_1: loop {
        loop {
            loop {
                counter_1 += counter_2 + counter_3 + 1;
                counter_1 += counter_2 * counter_3;
                if counter_1 >= 20 {
                    counter_2 += counter_3 + 1;
                    if counter_2 >= 20 {
                        counter_3 += 1;
                    }
                }
                println!("your counters are now {counter_1} {counter_2} {counter_3}");
                if counter_1 >= 100000 {
                    break 'label_1 counter_1
                }
            }
        }
    };

    println!("Your final result is now {var}");



    println!("Hello, world!");
}
