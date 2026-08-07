fn isPrime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    !(2..).take_while(|&x| x * x <= n).any(|x| n % x == 0)
}

pub fn sieve(n: usize) -> Vec<usize> {
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
