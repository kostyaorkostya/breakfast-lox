use thiserror::Error;

#[derive(Error, Debug)]
#[error("out of fuel")]
pub struct OutOfFuelError;

#[derive(Debug)]
pub enum Fuel {
    Infinite,
    Finite(u64),
}

impl Fuel {
    pub fn burn(&mut self) -> Result<(), OutOfFuelError> {
        match self {
            Self::Infinite => Ok(()),
            Self::Finite(0) => Err(OutOfFuelError),
            Self::Finite(x) => {
                *self = Self::Finite(*x - 1u64);
                Ok(())
            }
        }
    }
}
