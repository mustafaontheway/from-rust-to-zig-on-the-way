#[derive(Debug)]
enum Departments {

    Sales = 11,
    Marketing = 12,
    Accounting = 17
}

fn main() {

    let sales_dep_id = Departments::Sales;

    println!("Sales department ID: {:?}", sales_dep_id as u32); // Sales department ID: 11
}

