#[macro_use]
extern crate clap;

use std::{
    io::{
        self, Read,
    },
    borrow::Borrow,
};

mod ciphers;
mod utils;

fn main() {
    let matches = clap_app!(crypt =>
        (version: "0.1.0")
        (author: "dikuchan <dikuchan@yahoo.com>")
        (about: "Encrypt piped strings")
        (@subcommand atbash =>
            (@group action +required =>
                (@arg encrypt: -e --encrypt)
                (@arg decrypt: -d --decrypt)
            )
        )
        (@subcommand affine =>
            (@group action +required =>
                (@arg encrypt: -e --encrypt)
                (@arg decrypt: -d --decrypt)
            )
            (@arg a: +required +takes_value "Slope")
            (@arg b: +required +takes_value "Intercept")
        )
        (@subcommand bacon =>
            (@group action +required =>
                (@arg encrypt: -e --encrypt)
                (@arg decrypt: -d --decrypt)
            )
        )
        (@subcommand caesar =>
            (@group action +required =>
                (@arg encrypt: -e --encrypt)
                (@arg decrypt: -d --decrypt)
            )
            (@arg shift: +required +takes_value "Number of letter to shift")
        )
        (@subcommand rot13 =>
            (@group action +required =>
                (@arg encrypt: -e --encrypt)
                (@arg decrypt: -d --decrypt)
            )
        )
    ).get_matches();

    match matches.subcommand() {
        ("atbash", _) => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => {}
                Err(err) => eprintln!("Cannot read string: {}", err)
            }
            print!("{}", ciphers::atbash::encrypt(buffer.borrow()))
        }
        ("rot13", _) => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => {}
                Err(err) => eprintln!("Cannot read string: {}", err)
            }
            print!("{}", ciphers::rot13::encrypt(buffer.borrow()))
        }
        ("bacon", bacon_matches) => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Cannot read string: {}", err);
                    return;
                }
            }
            let matches = if let Some(matches) = bacon_matches {
                matches
            } else {
                return;
            };
            let (encrypt, decrypt) = (
                matches.is_present("encrypt"),
                matches.is_present("decrypt")
            );
            match (encrypt, decrypt) {
                (true, _) => {
                    match ciphers::bacon::encrypt(buffer.borrow()) {
                        Ok(string) => print!("{}", string),
                        Err(err) => {
                            eprintln!("{}", err);
                            print!("{}", buffer)
                        }
                    }
                }
                (_, true) => {
                    match ciphers::bacon::decrypt(buffer.borrow()) {
                        Ok(string) => print!("{}", string),
                        Err(err) => {
                            eprintln!("{}", err);
                            print!("{}", buffer)
                        }
                    }
                }
                _ => unreachable!()
            }
        }
        ("caesar", caesar_matches) => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Cannot read string: {}", err);
                    return;
                }
            }
            let matches = if let Some(matches) = caesar_matches {
                matches
            } else {
                return;
            };
            let shift = if let Some(shift) = matches.value_of("shift") {
                shift
            } else {
                eprintln!("Shift not specified");
                return;
            };
            let shift = match shift.parse::<u8>() {
                Ok(num) => num,
                Err(err) => {
                    eprintln!("Erroneous shift: {}", err);
                    return;
                }
            };
            let (encrypt, decrypt) = (
                matches.is_present("encrypt"),
                matches.is_present("decrypt")
            );
            match (encrypt, decrypt) {
                (true, _) => {
                    match ciphers::caesar::encrypt(buffer.borrow(), shift) {
                        Ok(string) => print!("{}", string),
                        Err(err) => {
                            eprintln!("{}", err);
                            print!("{}", buffer)
                        }
                    }
                }
                (_, true) => {
                    match ciphers::caesar::decrypt(buffer.borrow(), shift) {
                        Ok(string) => print!("{}", string),
                        Err(err) => {
                            eprintln!("{}", err);
                            print!("{}", buffer)
                        }
                    }
                }
                _ => unreachable!()
            }
        }
        ("affine", affine_matches) => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => {}
                Err(err) => eprintln!("Cannot read string: {}", err)
            }
            let matches = if let Some(matches) = affine_matches {
                matches
            } else {
                return;
            };
            let a = if let Some(a) = matches.value_of("a") {
                a
            } else {
                eprintln!("Slope not specified");
                return;
            };
            let a = match a.parse::<i64>() {
                Ok(num) => num,
                Err(err) => {
                    eprintln!("Erroneous slope: {}", err);
                    return;
                }
            };
            let b = if let Some(b) = matches.value_of("b") {
                b
            } else {
                eprintln!("Intercept not specified");
                return;
            };
            let b = match b.parse::<i64>() {
                Ok(num) => num,
                Err(err) => {
                    eprintln!("Erroneous intercept: {}", err);
                    return;
                }
            };
            let (encrypt, decrypt) = (
                matches.is_present("encrypt"),
                matches.is_present("decrypt")
            );
            match (encrypt, decrypt) {
                (true, _) => {
                    match ciphers::affine::encrypt(buffer.borrow(), a, b) {
                        Ok(string) => print!("{}", string),
                        Err(err) => {
                            eprintln!("{}", err);
                            print!("{}", buffer)
                        }
                    }
                }
                (_, true) => {
                    match ciphers::affine::decrypt(buffer.borrow(), a, b) {
                        Ok(string) => print!("{}", string),
                        Err(err) => {
                            eprintln!("{}", err);
                            print!("{}", buffer)
                        }
                    }
                }
                _ => unreachable!()
            }

        }
        ("", None) => return,
        _ => unreachable!()
    }
}
