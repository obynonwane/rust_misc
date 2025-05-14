/*
   Need to get greatest the common diviser of a given number e.g (12, 8) -> 4
*/

// define the function
fn gcd(mut n: i32, mut m: i32) -> i32 {
    assert!(m != 0 && n != 0);

    while m != 0 {
        // exchange the values of m, n if m < n
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m %= n;
    }
    n
}

use std::env;

#[test]
fn test_gcp() {
    assert_eq!(gcd(14, 15), 1);
}
fn main() {
    let args = env::args();

    for arg in args.skip(1) {
        println!("The arg from terminal {}", arg)
    }
    let res = gcd(10, 29);
    println!("The gcd is: {}", res)
}
