#![allow(clippy::let_and_return)]

pub mod round_float {
    use anyhow::Result;
    use num_traits::Float;

    /// Round `float_number` to specified number of digits in the fraction.
    /// # Errors
    /// Throws error if:
    /// * can't instantiate `10.0`
    /// * can't convert `digits` to a Float number.
    pub fn round_to_fraction<F: Float>(float_number: F, digits: u32) -> Result<F> {
        let ten = F::from(10.0).ok_or(anyhow::Error::msg("Failed to instantiate value `10.0`."))?;
        let digits =
            F::from(digits).ok_or(anyhow::Error::msg("Failed to convert `digits``to float"))?;
        let round_factor = ten * digits;
        let rounded_float = (float_number * round_factor).round() / round_factor;

        Ok(rounded_float)
    }
}
