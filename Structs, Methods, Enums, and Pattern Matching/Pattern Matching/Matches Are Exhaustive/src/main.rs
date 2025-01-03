﻿fn plus_one(x: Option<i32>) -> Option<i32> {
    // !!! ERROR: Match must be exhaustive
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}", six, none);
}
