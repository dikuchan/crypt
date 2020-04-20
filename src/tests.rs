#[allow(unused_imports)]
use crate::*;

#[test]
fn test_atbash_encryption() {
    assert_eq!(ciphers::atbash::encrypt("Attack at dawn"), Some(String::from("Zggzxp zg wzdm")));
    assert_eq!(ciphers::atbash::encrypt("true iS 42"), Some(String::from("gifv rH 42")));
    assert_eq!(ciphers::atbash::encrypt("こんばんは"), None);
    assert_eq!(ciphers::atbash::encrypt("Hello, world!"), Some(String::from("Svool, dliow!")))
}

#[test]
fn test_atbash_decryption() {
    assert_eq!(ciphers::atbash::encrypt("Zggzxp zg wzdm"), Some(String::from("Attack at dawn")));
    assert_eq!(ciphers::atbash::encrypt("gifv rH 42"), Some(String::from("true iS 42")));
    assert_eq!(ciphers::atbash::encrypt("こんばんは"), None);
}

#[test]
fn test_rot13_encryption() {
    assert_eq!(ciphers::rot13::encrypt("Attack at dawn"), Some(String::from("Nggnpx ng qnja")));
    assert_eq!(ciphers::rot13::encrypt("true iS 42"), Some(String::from("gehr vF 42")));
    assert_eq!(ciphers::rot13::encrypt("こんばんは"), None);
    assert_eq!(ciphers::rot13::encrypt("Hello, world!"), Some(String::from("Uryyb, jbeyq!")))
}

#[test]
fn test_rot13_decryption() {
    assert_eq!(ciphers::rot13::encrypt("Nggnpx ng qnja"), Some(String::from("Attack at dawn")));
    assert_eq!(ciphers::rot13::encrypt("gehr vF 42"), Some(String::from("true iS 42")));
    assert_eq!(ciphers::rot13::encrypt("こんばんは"), None);
}

#[test]
fn test_caesar_encryption() {
    assert_eq!(ciphers::caesar::encrypt("Attack at dawn", 5), Some(String::from("Fyyfhp fy ifbs")));
    assert_eq!(ciphers::caesar::encrypt("true iS 42", 5), Some(String::from("ywzj nX 42")));
    assert_eq!(ciphers::caesar::encrypt("こんばんは", 5), None);
    assert_eq!(ciphers::caesar::encrypt("Hello, world!", 25), Some(String::from("Gdkkn, vnqkc!")));
}

#[test]
fn test_caesar_decryption() {
    assert_eq!(ciphers::caesar::decrypt("Fyyfhp fy ifbs", 5), Some(String::from("Attack at dawn")));
    assert_eq!(ciphers::caesar::decrypt("ywzj nX 42", 5), Some(String::from("true iS 42")));
    assert_eq!(ciphers::caesar::decrypt("こんばんは", 5), None);
}
