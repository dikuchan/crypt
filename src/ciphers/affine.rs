use crate::utils;

fn egcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { egcd(b, a % b) }
}

pub fn encrypt(string: &str, a: i64, b: i64) -> Result<String, utils::CipherError> {
    let a = a % 0x1A;
    let b = b % 0x1A;

    if egcd(a, 0x1A) != 1 {
        return Err(utils::CipherError::NotPrime);
    }

    let result = string.chars()
        .map(move |c|
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

pub fn decrypt(string: &str, a: i64, b: i64) -> Result<String, utils::CipherError> {
    let a = if let Some(a) = invert(a, 0x1A) {
        a
    } else {
        return Err(utils::CipherError::NotPrime);
    };

    let result = string.chars()
        .map(move |c|
            match c {
                'a'..='z' => ((((a * (c as i64 - 0x61 - b)) % 0x1A + 0x1A) % 0x1A) as u8 + 0x61) as char,
                'A'..='Z' => ((((a * (c as i64 - 0x41 - b)) % 0x1A + 0x1A) % 0x1A) as u8 + 0x41) as char,
                _ => c
            }
        )
        .collect();

    Ok(result)
}

#[test]
fn test_affine_encryption() {
    assert_eq!(encrypt("Attack at dawn", 5, 8).unwrap(), String::from("Izzisg iz xiov"));
    assert_eq!(encrypt("true iS 42", 15, 9).unwrap(), String::from("iexr zT 42"));
    assert!(encrypt("こんばんは, mates", 14, 2).is_err());
    assert_eq!(encrypt("Привет, world!", 25, 37).unwrap(), String::from("Привет, pxuai!"))
}

#[test]
fn test_affine_decryption() {
    assert_eq!(decrypt("Izzisg iz xiov", 5, 8).unwrap(), String::from("Attack at dawn"));
    assert_eq!(decrypt("iexr zT 42", 15, 9).unwrap(), String::from("true iS 42"));
    assert!(decrypt("こんばんは, mates", 14, 2).is_err());
    assert_eq!(decrypt("Привет, pxuai!", 25, 37).unwrap(), String::from("Привет, world!"))
}