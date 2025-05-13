use std::env;
use std::str::FromStr;

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m > 0 && n > 0, "gcd requires positive integers");
    // Euclidean algorithm
    while n != 0 {
        if n < m {
            let t = n;
            n= m;
            m = t;
        }
        n = n % m;
    }
    m
}

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];

    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
