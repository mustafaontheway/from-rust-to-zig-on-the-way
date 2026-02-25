fn main() {

    let current_year: u16 = 2026;

    let age_calculator1 = |birth_year: u16| current_year - birth_year;

    println!("Age (u16): {}", age_calculator1(2000));

    let age_calculator2 = |birth_year: u16| (current_year - birth_year) as u8;

    println!("Age (u8): {}", age_calculator2(2000));
}

// Age (u16): 26
// Age (u8): 26
