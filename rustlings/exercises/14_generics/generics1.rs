// `Vec<T>` is generic over the type `T`. In most cases, the compiler is able to
// infer `T`, for example after pushing a value with a concrete type to the vector.
// But in this exercise, the compiler needs some help through a type annotation.

fn main() {
    // Call the helper function with the desired type
    process_numbers::<i32>();
}

fn process_numbers<T: std::fmt::Debug + std::convert::From<i8> + std::convert::From<u8>>() {
    let mut numbers: Vec<T> = Vec::new();

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
