fn main() {

    let current_year: u16 = 2026;

    let age_calculator = |birth_year: u16| -> u8 {

        //return  (current_year - birth_year) as u8;
        (current_year - birth_year) as u8
    };

    println!("Age: {}", age_calculator(1912)) // Age: 114
}



// cargo run main.rs
