pub fn calculate_price(apples_to_order: i32) -> i32 {
    let mut price_per_apple = 2;
    if apples_to_order >= 40 {
        price_per_apple = 1;
    }
    let total_price = apples_to_order * price_per_apple;
    return total_price;
    // let i = total_price;
    // i
}

