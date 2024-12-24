use errors5::*;
use std::error::Error;
use std::num::ParseIntError;
use std::str::Matches;

// Update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    let y = match PositiveNonzeroInteger::new(x) {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };
    println!("output={:?}", y);
    Ok(())
}