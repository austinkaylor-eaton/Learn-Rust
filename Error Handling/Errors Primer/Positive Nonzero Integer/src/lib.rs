﻿#[derive(PartialEq, Debug)]
pub struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
pub enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    pub fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value { 
            value if value > 0 => Ok(PositiveNonzeroInteger(value as u64)),
            0 => Err(CreationError::Zero),
            value if value < 0 => Err(CreationError::Negative),
            _ => Err(CreationError::Negative),
        }
    }
}
