#![allow(clippy::let_and_return)]

use num_traits::Float;
pub trait RoundToFraction {
    /// Round `float_number` to specified number of digits in the fraction.
    /// # Errors
    /// Throws error if `digits` is zero
    fn round_to_fraction(&self, digits: u32) -> Self
    where
        Self: Float,
    {
        let rounded_float = if digits == 0 {
            let rounded_float = self.trunc();
            rounded_float
        } else {
            let ten = Self::from(10.0).unwrap();
            let digits = Self::from(digits).unwrap();
            let round_factor = ten * digits;
            let rounded_float = (self.mul(round_factor)).round() / round_factor;
            rounded_float
        };

        rounded_float
    }
}

impl<F> RoundToFraction for F
where
    F: Float,
{
    fn round_to_fraction(&self, digits: u32) -> Self {
        let rounded_float = if digits == 0 {
            let rounded_float = self.trunc();
            rounded_float
        } else {
            let ten = Self::from(10.0).unwrap();
            let digits = Self::from(digits).unwrap();
            let round_factor = ten * digits;
            let rounded_float = (self.mul(round_factor)).round() / round_factor;
            rounded_float
        };

        rounded_float
    }
}
