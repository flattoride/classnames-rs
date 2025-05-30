// Add crate-level attributes
#![doc(html_root_url = "https://docs.rs/classnames-const/0.1.0")]
#![deny(missing_docs)]
// Remove unsafe forbid since we use unsafe transmute
// #![forbid(unsafe_code)]

//! A Rust macro library for compile-time CSS class name concatenation and processing
//!
//! # Features
//!
//! - Zero runtime overhead - everything happens at compile time
//! - Automatic whitespace handling and normalization
//! - Support for concatenating multiple class names
//! - Removes leading/trailing whitespace and collapses multiple spaces
//!
//! # Examples
//!
//! Basic usage:
//! ```rust
//! use classnames_const_rs::*;
//!
//! const BUTTON_CLASS: &str = classnames_concat!("btn", "btn-primary");
//! assert_eq!(BUTTON_CLASS, "btn btn-primary");
//! ```
//!
//! Handling messy whitespace:
//! ```rust
//! use classnames_const_rs::*;
//!
//! const COMPLEX_CLASS: &str = classnames_concat!("header  ", "  main  ", "footer");
//! assert_eq!(COMPLEX_CLASS, "header main footer");
//! ```
//!
//! Empty strings are handled gracefully:
//! ```rust
//! use classnames_const_rs::*;
//!
//! const SPARSE_CLASS: &str = classnames_concat!("", "active", "", "highlight");
//! assert_eq!(SPARSE_CLASS, "active highlight");
//! ```

/// Concatenates multiple class name strings with automatic whitespace handling
///
/// This macro performs compile-time concatenation of class names, ensuring that:
/// - Class names are separated by single spaces
/// - Leading and trailing whitespace is removed
/// - Multiple consecutive spaces are collapsed to single spaces
///
/// # Examples
///
/// ```rust
/// use classnames_const_rs::{classnames_concat, trim_format};;
///
/// const CLASSES: &str = classnames_concat!("btn", "btn-large", "btn-primary");
/// assert_eq!(CLASSES, "btn btn-large btn-primary");
///
/// const MESSY_CLASSES: &str = classnames_concat!("  header ", " main  ", "footer  ");
/// assert_eq!(MESSY_CLASSES, "header main footer");
/// ```
#[macro_export]
macro_rules! classnames_concat {
    ($($x:expr),* $(,)?) => {
        trim_format!(constcat::concat!($($x, " "),*))
    };
}

/// Formats a string by normalizing whitespace characters
///
/// This macro processes strings at compile time to:
/// - Replace consecutive whitespace characters with single spaces
/// - Remove leading and trailing whitespace
/// - Handle various whitespace characters (spaces, tabs, newlines, etc.)
///
/// This macro is primarily used internally by `classnames_concat` and typically
/// doesn't need to be called directly.
///
/// # Examples
///
/// ```rust
/// use classnames_const_rs::*;
///
/// const NORMALIZED: &str = trim_format!("  hello    world  ");
/// assert_eq!(NORMALIZED, "hello world");
/// ```
#[macro_export]
macro_rules! trim_format {
    ($input:expr) => {{
        {
            use ::constcat::core::mem;
            use ::constcat::core::primitive::{str, u8};

            const fn is_whitespace(c: u8) -> bool {
                if let Some(ch) = char::from_u32(c as u32) {
                    ch.is_whitespace()
                } else {
                    false
                }
            }

            const SRC: &[u8] = $input.as_bytes();
            const LEN: usize = $input.len();

            const ARR: [u8; LEN] = {
                let mut result = [0u8; LEN];
                let mut pos = 0;
                let mut i = 0;
                let mut last_was_space = true; // Skip leading whitespace

                // Copy and process characters
                while i < LEN {
                    if is_whitespace(SRC[i]) {
                        if !last_was_space {
                            result[pos] = b' ';
                            pos += 1;
                            last_was_space = true;
                        }
                    } else {
                        result[pos] = SRC[i];
                        pos += 1;
                        last_was_space = false;
                    }
                    i += 1;
                }

                // Remove trailing space if present
                if pos > 0 && result[pos - 1] == b' ' {
                    pos -= 1;
                }

                // Fill remaining space with zeros
                while pos < LEN {
                    result[pos] = 0;
                    pos += 1;
                }

                result
            };

            // Calculate actual length (up to first zero or end of array)
            const REAL_LEN: usize = {
                let mut len = 0;
                while len < LEN && ARR[len] != 0 {
                    len += 1;
                }
                len
            };

            // Create final fixed-size array for the result
            const FINAL: [u8; REAL_LEN] = {
                let mut result = [0u8; REAL_LEN];
                let mut i = 0;
                while i < REAL_LEN {
                    result[i] = ARR[i];
                    i += 1;
                }
                result
            };

            unsafe {
                mem::transmute::<&[u8], &str>(&FINAL)
            }
        }
    }};
}

