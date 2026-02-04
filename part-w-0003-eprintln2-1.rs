fn main() {

    let sales: u32 = 525_000;

    let fixed_cost: u32 = 600_000;

    let sales_result = sales as i32 - fixed_cost as i32;

    if sales_result < 0 {
        
        eprintln!("We need profit! Sales must exceed fixed cost! Total loss: {sales_result}") // We need profit! Sales must exceed fixed cost! Total loss: -75000

    } else {

        println!("Profit amount: {}", sales_result)
    }

    println!("Calculation ended...")
}
