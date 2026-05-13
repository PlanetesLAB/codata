# codata

A lightweight, `no_std` Rust crate providing CODATA 2022 physical constants and IAU 2015 astronomical constants.

## Usage

### Direct Access & Aliases

Familiar symbols are available in the `aliases` module.

```rust
use codata::aliases::{C, G_SI, G_CGS, AU, M_SUN};

fn main() {
    println!("Speed of light: {} m/s", C);
    println!("G (SI):  {}", G_SI);
    println!("G (CGS): {}", G_CGS);
    println!("Solar Mass: {} kg", M_SUN);
}
```

### Dynamic Lookup

Search constants by their official NIST/IAU name.

```rust
use codata::find;

fn main() {
    if let Some(c) = find("speed of light in vacuum") {
        println!("Value: {} {}", c.value, c.unit);
    }
}
```

### Unit Conversions

```rust
use codata::units::length::LIGHT_YEAR;
use codata::units::time::HOUR;

let dist_m = 5.0 * LIGHT_YEAR;
let duration_s = 2.0 * HOUR;
```

## Data Sources

- **Physics**: [CODATA 2022 (NIST)](https://physics.nist.gov/cuu/Constants/Table/allascii.txt)
- **Astronomy**: [Official IAU Resolutions](https://www.iau.org/publications/proceedings_rules/resolutions/) (specifically 2012 B2 and 2015 B3).

## License

MIT OR Apache-2.0
