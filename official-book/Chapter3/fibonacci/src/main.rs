fn main() {

    let mut fib : (i128, i128, i128) = (0, 1, 1);
    let mut tmp : i128;

    println!("The first three numbers of the fibonacci sequence are {}, {}, {}", fib.0, fib.1, fib.2);

    for counter in 3..101 {
        tmp = fib.2;
        fib.2 = fib.1 + fib.0;
        fib.0 = fib.1;
        fib.1 = tmp;

        println!("The {counter}-{} number of the fibonacci sequence is {}",
            if counter%10==0 || counter%10>3 { "th"
            } else if counter%10==1 { "st" 
            } else if counter%10==2 { "nd" 
            } else { "rd" }, 
            fib.2);
    }

}
