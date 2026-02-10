fn main() {

    println!("{}", factorial(5));

    println!("{}", factorial(15));

    println!("{}", factorial(0));
}

fn factorial(x: u128) -> u128 {

    if x == 0 { return 1;}

    x * factorial(x - 1)
}

// fn factorial(x: u128) -> u128 {

//     let mut r: u128 = 1;

//     for i in 1..= x {

//         r *= i;
//     }

//     r
// }

// 120
// 1307674368000
// 1
