fn main() {

    let person_Ayhan = Person {name: "Ayhan Bİlir", actual_age: 22.5};

    println!("Name: {} and age: {}", person_Ayhan.name, person_Ayhan.actual_age);

    let person_bilge: Person<u8> = Person {name: "Ayhan Bİlir", actual_age: 35};

    println!("Name: {} and age: {}", person_bilge.name, person_bilge.actual_age);
}

struct Person<T> {

    name: &'static str,
    actual_age: T
}

// Name: Ayhan Bİlir and age: 22.5
// Name: Ayhan Bİlir and age: 35
