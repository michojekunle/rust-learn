fn main() {
    println!("Hello, world!");
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let rem = a % b;
        a = b;
        b = rem;
    }
    a
}

fn isPrime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    !(2..).take_while(|&x| x * x <= n).any(|x| n % x == 0)
}

fn sieve(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; (n + 1) as usize];

    let mut p = 2;

    while p * p < n {
        if is_prime[p as usize] {
            let mut multiple = p * p;

            while multiple <= n {
                is_prime[multiple as usize] = false;
                multiple += p;
            }
        }
        p += 1;
    }

    let mut primes = Vec::new();

    for i in 2..=n {
        if is_prime[i as usize] {
            primes.push(i);
        }
    }

    primes
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
