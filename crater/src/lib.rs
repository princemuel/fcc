//! # Crater
//!
//! `crater` is a collection of utilities to make
//! perfoming certain caluclations more convenient

/// Adds one to the input number
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = crater::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
