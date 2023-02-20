//! # SlothNum
//! 
//! The big number library created for and used by the programming language [Sloth](https://crates.io/crates/slothlang).
//! 
//! The main struct is [BigNum]. In most of the cases, you won't need to use [core].

mod macros;
pub mod core;
mod big_num;

pub use big_num::BigNum;