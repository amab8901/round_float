#![allow(clippy::let_and_return)]

use core::fmt::Debug;

use num_traits::Float;
pub trait RoundToFraction {
    /// Round `float_number` to specified number of digits in the fraction.
    fn round_to_fraction(&self, digits: u32) -> Self
    where
        Self: Float + Debug,
    {
        let rounded_float = if digits == 0 {
            let rounded_float = self.trunc();
            rounded_float
        } else {
            let ten = Self::from(10.0).unwrap();
            let digits = Self::from(digits).unwrap();
            let round_factor = ten.powf(digits);
            let rounded_float = (self.mul(round_factor)).round() / round_factor;
            rounded_float
        };

        rounded_float
    }
}

impl<F> RoundToFraction for F where F: Float {}

#[cfg(test)]
mod tests {
    use super::RoundToFraction;

    #[test]
    fn five_digits() {
        let before = 100.123456789;
        let after = before.round_to_fraction(5);
        assert_eq!(after, 100.12346)
    }
}
