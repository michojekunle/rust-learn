mod ch_01;
use ch_01::{gcd::gcd, sieve::sieve};

fn main() {
    println!("Hello, world!");
}

#[test]
fn sieve_test_cases() {
    let cases = [
        (2, vec![2]),
        (10, vec![2, 3, 5, 7]),
        (20, vec![2, 3, 5, 7, 11, 13, 17, 19]),
    ];

    for (limit, expected) in cases {
        assert_eq!(sieve(limit), expected);
    }
}

#[test]
fn gcd_test_cases() {
    let cases = [
        ((48, 18), 6),
        ((270, 192), 6),
        ((17, 13), 1),
        ((0, 5), 5),
        ((5, 0), 5),
        ((42, 42), 42),
    ];

    for ((a, b), expected) in cases {
        assert_eq!(gcd(a, b), expected);
    }
}
