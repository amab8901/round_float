This crate extends your float numbers (`f64` and `f32`) with the trait method `round_to_fraction()`, which lets you round the float number to a specified number of fraction digits.

# Example

```
use round_float::RoundToFraction;

let full_float = 12.34567;
let rounded_float = full_float.round_to_fraction(2);

assert_eq!(rounded_float, 12.35);
```