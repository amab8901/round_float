use anyhow::Result;
use num_traits::Float;

/// Round `float_number` to specified number of digits in the fraction.
/// # Errors
/// Throws error if:
/// * can't instantiate `10.0`
/// * can't convert `digits` to a Float number
/// * `digits` is zero
pub fn round_to_fraction<F: Float>(float_number: F, digits: u32) -> Result<F> {
    if digits == 0 {
        return Err(anyhow::Error::msg("`digits` must be a positive integer"));
    }
    let ten = F::from(10.0).ok_or(anyhow::Error::msg("Failed to instantiate value `10.0`."))?;
    let digits =
        F::from(digits).ok_or(anyhow::Error::msg("Failed to convert `digits``to float"))?;
    let round_factor = ten * digits;
    let rounded_float = (float_number * round_factor).round() / round_factor;

    Ok(rounded_float)
}

pub trait RoundToFraction {
    /// Round `float_number` to specified number of digits in the fraction.
    /// # Errors
    /// Throws error if:
    /// * can't instantiate `10.0`
    /// * can't convert `digits` to a Float number
    /// * `digits` is zero
    fn round_to_fraction<F: Float>(float_number: F, digits: u32) -> Result<F> {
        if digits == 0 {
            return Err(anyhow::Error::msg("`digits` must be a positive integer"));
        }
        let ten = F::from(10.0).ok_or(anyhow::Error::msg("Failed to instantiate value `10.0`."))?;
        let digits =
            F::from(digits).ok_or(anyhow::Error::msg("Failed to convert `digits``to float"))?;
        let round_factor = ten * digits;
        let rounded_float = (float_number * round_factor).round() / round_factor;

        Ok(rounded_float)
    }
}

impl<T> RoundToFraction for T
where
    T: Float,
{
    fn round_to_fraction<F: Float>(float_number: F, digits: u32) -> Result<F> {
        if digits == 0 {
            return Err(anyhow::Error::msg("`digits` must be a positive integer"));
        }
        let ten = F::from(10.0).ok_or(anyhow::Error::msg("Failed to instantiate value `10.0`."))?;
        let digits =
            F::from(digits).ok_or(anyhow::Error::msg("Failed to convert `digits``to float"))?;
        let round_factor = ten * digits;
        let rounded_float = (float_number * round_factor).round() / round_factor;

        Ok(rounded_float)
    }
}
