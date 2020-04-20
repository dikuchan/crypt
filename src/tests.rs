#[allow(unused_imports)]
use crate::*;

#[test]
fn test_atbash_encryption() {
    assert_eq!(atbash::encrypt("Attack at dawn"), Some(String::from("Zggzxp zg wzdm")));
    assert_eq!(atbash::encrypt("true iS 42"), Some(String::from("gifv rH 42")));
    assert_eq!(atbash::encrypt("こんばんは"), None);
    assert_eq!(atbash::encrypt("Hello, world!"), Some(String::from("Svool, dliow!")))
}

#[test]
fn test_atbash_decryption() {
    assert_eq!(atbash::encrypt("Zggzxp zg wzdm"), Some(String::from("Attack at dawn")));
    assert_eq!(atbash::encrypt("gifv rH 42"), Some(String::from("true iS 42")));
    assert_eq!(atbash::encrypt("こんばんは"), None);
}

#[test]
fn test_rot13_encryption() {
    assert_eq!(rot13::encrypt("Attack at dawn"), Some(String::from("Nggnpx ng qnja")));
    assert_eq!(rot13::encrypt("true iS 42"), Some(String::from("gehr vF 42")));
    assert_eq!(rot13::encrypt("こんばんは"), None);
    assert_eq!(rot13::encrypt("Hello, world!"), Some(String::from("Uryyb, jbeyq!")))
}

#[test]
fn test_rot13_decryption() {
    assert_eq!(rot13::encrypt("Nggnpx ng qnja"), Some(String::from("Attack at dawn")));
    assert_eq!(rot13::encrypt("gehr vF 42"), Some(String::from("true iS 42")));
    assert_eq!(rot13::encrypt("こんばんは"), None);
}

#[test]
fn test_caesar_encryption() {
    assert_eq!(caesar::encrypt("Attack at dawn", 5), Some(String::from("Fyyfhp fy ifbs")));
    assert_eq!(caesar::encrypt("true iS 42", 5), Some(String::from("ywzj nX 42")));
    assert_eq!(caesar::encrypt("こんばんは", 5), None);
    assert_eq!(caesar::encrypt("Hello, world!", 25), Some(String::from("Gdkkn, vnqkc!")));
}

#[test]
fn test_caesar_decryption() {
    assert_eq!(caesar::decrypt("Fyyfhp fy ifbs", 5), Some(String::from("Attack at dawn")));
    assert_eq!(caesar::decrypt("ywzj nX 42", 5), Some(String::from("true iS 42")));
    assert_eq!(caesar::decrypt("こんばんは", 5), None);
}
