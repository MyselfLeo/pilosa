//! # SlothNum
//! 
//! The big number library created for and used by the programming language [Sloth](https://crates.io/crates/slothlang).


pub mod core;
mod big_num;

mod macros;

pub use big_num::BigNum;