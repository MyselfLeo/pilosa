# Pilosa
A big number library. Its main feature is `BigNum`, a structure representing an arbitrary long/precise decimal number.

## Usage
You can create a `BigNum` either from a string, an i32 or a f64:
```rust
use pilosa::BigNum;

let n1 = BigNum::from_string("3536.213").unwrap();
let n2 = BigNum::from_f64(3545.57458).unwrap();
let n3 = BigNum::from_i32(-242952842).unwrap();
```

The `BigNum` structure implements common operations and comparaison traits, so you can use them like any other variable:

```rust
use pilosa::BigNum;

let n1 = BigNum::from_string("53643.368359").unwrap();
let n2 = BigNum::from_string("24872398247.24982").unwrap();

let n3 = n1 - n2;
assert_eq!(n3, BigNum::from_string("-24872344603.881461").unwrap());
```

`BigNum` also has a `bn_pow` function used to compute the power of a `BigNum` to an integer.


## Note
This library was not made by a professionnal. The algorithms used are not the best and they may be too slow / error prone for certain use cases. Please keep that in mind. Sorry!


## Installation
You can clone this repository and use it in your own projects (see [Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) from The Cargo Book). I do not plan to make it available on `crates.io` for now.


## License
Pilosa is licensed under **Mozilla Public License 2.0**. See `LICENSE.txt`.