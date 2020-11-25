use crate::error::*;

/// Check if string contains only alphabetic letters, numbers and some special ASCII characters.
pub fn is_alpha(text: &str) -> bool {
    text.chars().all(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace())
}

/// Return the greatest common divisor of two integers.
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Extended GCD algorithm.
///
/// # Arguments
/// * `a`, `b` - arbitrary numbers.
///
/// # Return
/// * (`x`, `y`, `d`) - a triple, where
///     * `x` and `y` - integers satisfying the equation `ax` + `by` = gcd(`a`, `b`).
///     * `d` - gcd(`a`, `b`).
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    match a {
        0 => (b, 0, 1),
        _ => {
            let (d, x, y) = egcd(b % a, a);
            (d, y - (b / a) * x, x)
        }
    }
}

/// Find modular multiplicative inverse.
///
/// # Arguments
/// `a`, `m` - integers such that `ax`= 1 (mod `m`).
///
/// # Return
/// `x` - modular inverse.
pub fn inverse(a: i64, m: i64) -> Result<i64> {
    let (d, x, _) = egcd(a, m);
    match d {
        1 => Ok((x % m + m) % m),
        _ => Err(CipherError::InvalidInverse(a, m)),
    }
}
