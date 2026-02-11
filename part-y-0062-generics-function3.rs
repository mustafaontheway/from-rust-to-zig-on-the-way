use std::ops::{Add, Mul};

fn main() {

    let r1: (u8, u8) = calculate_sum_or_mult(3, 5);

    println!("{:?}", r1); // (8, 15)

    let (sum, mult): (f32, f32) = calculate_sum_or_mult(3.14, 2.21);

    println!("{}", sum); // 5.3500004

    println!("{}", mult); // 6.9394

}

fn calculate_sum_or_mult<T>(a: T, b: T) -> (T, T)
    where
        T: Add<Output = T> + Mul<Output = T> + Copy
{

    (a + b, a * b)
}

//consider restricting type parameter `T` with trait `Mul`: `: std::ops::Mul<Output = T>

// cargo run main.rs
