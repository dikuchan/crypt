/*
 *  [Algorithm]
 *
 *  The key for the Affine cipher consists of 2 numbers, we'll call them a and b.
 *  The following discussion assumes the use of a 26 character alphabet (m = 26).
 *  a should be chosen to be relatively prime to m (i. e. a should have no factors in common with m).
 *  When encrypting, we first convert all the letters to numbers ('a' = 0, 'b' = 1, ..., 'z' = 25).
 *  The ciphertext letter c, for any given letter p is c = ap + b (mod m), 1 <= b <= m.
 *  The decryption function is
 *
 *  p = a^(-1) (c - b) (mod m),
 *
 *  where a^(−1) is the multiplicative inverse of a in the group of integers modulo m.
 *  To find a multiplicative inverse, we need to find x such that if we find the number x such that the equation is true,
 *  then x is the inverse of a, and we call it a^(−1).
 *  The easiest way to solve equation is to search each of the numbers 1 to 25, and see which one satisfies the equation.
 *
 *  [g, x, d] = gcd(a, m) % g and d could be ignored
 *  x = mod(x, m);
 *
 *  If you now multiply x and a and reduce the result (mod 26), you will get the answer 1.
 *  We now use the value of x we calculated as a^(-1).
 *  This allows us to perform the decryption step.
 *
 *  [Example]
 *
 *  Assume we discard all non alphabetical characters including spaces.
 *  Let the key be a = 5 and b = 7.
 *  The encryption function is then (5 * p + 7) (mod 26).
 *  To encode 'defend the east wall of the castle',
 *  we would take the first letter, 'd', convert it to a number, 3 and plug it into the equation:
 *
 *  c = (5 * p + 7) (mod 26) = (5 * 3 + 7) (mod 26) = 22,
 *
 *  since 'w' = 22, 'd' is transformed into 'w' using the values a = 5 and b = 7.
 *  If we continue with all the other letters we would have 'wbgbuw yqb bhty nhkk zg yqb rhtykb'.
 *  Now to decode, the inverse of 5 modulo 26 is 21, i. e. 5 * 21 = 1 (mod 26).
 *  The decoding function is
 *
 *  p = 21 * (c - 7) (mod 26) = 21 * (22 - 7) (mod 26) = 3,
 *
 *  so we have recovered d = 3 as the first plaintext character.
 *  Finally, we have 'defend the east wall of the castle'.
 *
 *  [Requirements]
 *
 *  All letter should be alpha characters.
 */

// Source: http://practicalcryptography.com

use crate::utils::*;

fn egcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { egcd(b, a % b) }
}

pub fn encrypt(string: &str, a: i64, b: i64) -> Result<String, CipherError> {
    if !is_alpha(string) {
        return Err(CipherError::NotAlpha);
    }

    let a = a % 0x1A;
    let b = b % 0x1A;

    if egcd(a, 0x1A) != 1 {
        return Err(CipherError::NotPrime);
    }

    let result = string.chars()
        .map(|c|
            match c {
                'a'..='z' => (((a * (c as i64 - 0x61) + b) % 0x1A) as u8 + 0x61) as char,
                'A'..='Z' => (((a * (c as i64 - 0x41) + b) % 0x1A) as u8 + 0x41) as char,
                _ => c
            })
        .collect();

    Ok(result)
}

fn dgcd(a: i64, b: i64) -> (i64, i64, i64) {
    match a {
        0 => (b, 0, 1),
        _ => {
            let (g, x, y) = dgcd(b % a, a);
            (g, y - (b / a) * x, x)
        }
    }
}

fn invert(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = dgcd(a, m);
    match g {
        1 => Some((x % m + m) % m),
        _ => None
    }
}

pub fn decrypt(string: &str, a: i64, b: i64) -> Result<String, CipherError> {
    if !is_alpha(string) {
        return Err(CipherError::NotAlpha);
    }

    let a = if let Some(a) = invert(a, 0x1A) {
        a
    } else {
        return Err(CipherError::NotPrime);
    };

    let result = string.chars()
        .map(|c|
            match c {
                'a'..='z' => ((((a * (c as i64 - 0x61 - b)) % 0x1A + 0x1A) % 0x1A) as u8 + 0x61) as char,
                'A'..='Z' => ((((a * (c as i64 - 0x41 - b)) % 0x1A + 0x1A) % 0x1A) as u8 + 0x41) as char,
                _ => c
            })
        .collect();

    Ok(result)
}

#[test]
fn test_affine_encryption() {
    assert_eq!(encrypt("Attack at dawn", 5, 8).unwrap(), String::from("Izzisg iz xiov"));
    assert_eq!(encrypt("true iS None", 15, 9).unwrap(), String::from("iexr zT Wlwr"));
    assert!(encrypt("こんばんは, mates", 14, 2).is_err());
    assert!(encrypt("Привет, world!", 25, 37).is_err())
}

#[test]
fn test_affine_decryption() {
    assert_eq!(decrypt("Izzisg iz xiov", 5, 8).unwrap(), String::from("Attack at dawn"));
    assert_eq!(decrypt("iexr zT Wlwr", 15, 9).unwrap(), String::from("true iS None"));
    assert!(decrypt("こんばんは, mates", 14, 2).is_err());
    assert!(decrypt("Привет, pxuai!", 25, 37).is_err())
}
