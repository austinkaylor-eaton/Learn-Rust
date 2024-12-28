pub fn factorial(num: u64) -> u64 {
    let num_iter = 1..=num;
    num_iter.product()
}
