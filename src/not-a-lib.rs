//! # Rust Bitcoin Cash Library
//!
//! This is a library for which supports the Bitcoin Cash network protocol and associated
//! primitives. It is designed for Rust programs built to work with the Bitcoin Cash network.
//!
//! It is also written entirely in Rust to illustrate the benefits of strong type
//! safety, including ownership and lifetime, for financial and/or cryptographic
//! software.

/// Foobar function
pub fn foobar(str: &str) {
    println!("foobar");
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
