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

// use core::num;
use std::env;
use std::str::FromStr;

#[test]
fn test_gcp() {
    assert_eq!(gcd(14, 15), 1);
}
fn main() {
    //we use vector to save all arguments from command line
    let mut numbers = Vec::new();
    //get arguments from command line but igonre the first one
    //becuase it is the name of our executbale
    let args = env::args().skip(1);
    for arg in args {
        //:: means class method or static method
        let result = i32::from_str(&arg);
        match result {
            Ok(num) => {
                numbers.push(num);
            }
            Err(_e) => {
                //print the error message as err on console
                eprintln!("error parsing arguemnt to number");
                //exit from the app
                std::process::exit(1);
            }
        }
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    //iterate begin from the second element in the vector
    for m in &numbers[1..] {
        //reference and dereference
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
