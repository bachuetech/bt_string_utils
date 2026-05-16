//! Multiple String related functions
pub mod finder;
pub mod cleanser;
pub mod splitter;
pub mod analyzer;

use rand::distr::SampleString;
use rand::distr::Alphanumeric;

/// Generates a random URL-safe string of the specified length.
///
/// # Arguments
///
/// * `n` - The length of the generated string.
///
/// # Returns
///
/// A `String` containing `n` random alphanumeric characters (`A-Z`, `a-z`, `0-9`).
///
/// # Examples
///
/// ```
/// use bt_string_utils::generate_url_safe_string;
/// let random_string = generate_url_safe_string(16);
/// println!("Generated string: {}", random_string);
/// ```
///
/// # Notes
///
/// - Uses the `rand` crate to generate random alphanumeric characters.
/// - Ensures the output contains only **URL-safe** characters.
/// - May require the `rand` crate in your Cargo.toml:
///
pub fn generate_url_safe_string(n: usize) -> String {
    Alphanumeric.sample_string(&mut rand::rng(), n)       
}















