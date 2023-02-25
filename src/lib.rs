//! # Pilosa
//! 
//! A big number library
//! 
//! The main struct is [BigNum]. In most of the cases, you won't need to use [core].

mod macros;
pub mod core;
mod big_num;

pub use big_num::BigNum;
pub use big_num::FLOAT_PRECISION;