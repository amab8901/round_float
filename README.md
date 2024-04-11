This crate provides the function `round_to_fraction()`, which lets you round a float number to the specified number of fractions.

# Example

```
let full_float = 12.34567;
let rounded_float = round_to_fraction(full_float, 2);
assert_eq!(rounded_float, 12.35);
```