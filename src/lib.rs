#![crate_name = "rust_test_project"]

/**
 * Multiline comment?
 * Yeah.
*/

const SECRET_SAUCE: i64 = 1;

/// Adds the secret sauce. Mmmm.
/// 
/// # Arguments
/// * `n` 64-bit number
/// 
/// # Examples
/// ```
/// use rust_test_project::add_constant;
/// let result = add_constant(4);
/// assert_eq!(result, 5);
/// ```
/// ```
/// use rust_test_project::add_constant;
/// let result = add_constant(40000000000);
/// assert_eq!(result, 40000000001);
/// ```
pub fn add_constant(n: i64) -> i64 {
    return n+SECRET_SAUCE;
}
