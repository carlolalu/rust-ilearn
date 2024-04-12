/*Your task is to construct a building which will be a pile of n cubes. The cube at the bottom will have a volume of n^3, the cube above will have volume of (n−1)^3 and so on until the top which will have a volume of 1^3.

You are given the total volume m of the building. Being given m can you find the number n of cubes you will have to build?

The parameter of the function findNb (find_nb, find-nb, findNb, ...) will be an integer m and you have to return the integer n such as n^3 + (n-1)^3 + (n-2)^3 + ... + 1^3 = m if such a n exists or -1 if there is no such n.
Examples:

findNb(1071225) --> 45

findNb(91716553919377) --> -1

fn find_nb(m: u64) -> i32 {
    todo!();
}
*/

// the solution is based on the formula, which can be calculated, that:
// 4*SUM[from k=1 to k=n](k^3) = n^4 + 2*n^3 + n^2
// notice that (m==SUM) iff 4m/(n^2) is integer

fn pile4times(n : u64) -> u64 {
    let pile4 = n.pow(4) + 2*n.pow(3) + n.pow(2);
    pile4
}

fn find_nb(m : u64) -> i32 {

    match m {
        0 => return 0,
        1 => return 1,
        _other => (),
    }

    let mut n : u64 = 2;

    while 4*m > pile4times(n) {
        n += 1;
    }

    match 4*m == pile4times(n) {
        true => return n as i32,
        false => return -1,
    }
}


fn main() {
    println!("Hello, world!");

    for n in 1..=50 {
        println!("The pile of cubes of n={} has volume={}", n, pile4times(n)/4);
    }
    
    for m in 1..1700000 {
        let n = findNb(m);
        if n!=-1 {
            println!("findNb({}) is {}", m, n);
        }      
    }
}
