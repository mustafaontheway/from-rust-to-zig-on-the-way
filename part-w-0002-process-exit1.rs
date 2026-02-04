use std::process;

enum SalesErrorCodes {
    CostExceedSales = 101,
    SalesDataNotAvailable = 102
}

fn main() {

    let sales: u32 = 225_000;

    let fixed_cost: u32 = 600_000;

    if sales < fixed_cost {

        process::exit(SalesErrorCodes::CostExceedSales as i32);
    }

    if sales == 0 {

        process::exit(SalesErrorCodes::SalesDataNotAvailable as i32);
    }

    println!("Profit amount: {}", sales - fixed_cost)

}

