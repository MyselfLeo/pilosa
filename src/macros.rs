//! macros used in the codebase


/// Assert that the given condition is true.  
/// If not, return the given error msg as an error.  
/// Of course, this macro can only be used in a function with the return type `Result<_, String>`
#[macro_export]
macro_rules! assert_err {
    ($condition:expr, $($arg:tt)+) => {
        if (!$condition) {return Err(format!($($arg)+));}
    }
}