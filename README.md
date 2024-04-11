This crate provides the function `round_to_fraction()`, which lets you round a float number to the specified number of fractions.

# Example

```
let full_float = 12.34567;
let rounded_float = full_float.round_to_fraction(2);
assert_eq!(rounded_float, 12.35);
```