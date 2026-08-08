pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let rem = a % b;
        a = b;
        b = rem;
    }
    a
}
