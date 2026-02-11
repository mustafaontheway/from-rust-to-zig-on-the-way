use std::ops::{Add, Mul, Sub, Div};

fn main() {

    println!("{:?}", calculator(3.0, 0.0));

    println!("{:?}", calculator(7, 2));
}

fn calculator<T>(a: T, b: T) -> (T, T, T, T)

    where
        T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T> + Copy + Default + PartialEq
{
    let zero = T::default();

    if b == zero {

        println!("Zero division error!");
        
        return (zero, zero, zero, zero);
        
    } else {

        (a+ b, a - b, a * b, a / b)
    }
}

// (0.0, 0.0, 0.0, 0.0)
// (9, 5, 14, 3)


// cargo run main.rs
