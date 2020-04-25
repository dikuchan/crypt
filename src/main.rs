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
        (about: "Cipher piped strings")
        (@subcommand atbash =>
            (@group action +required =>
                (@arg encode: -e --encode)
                (@arg decode: -d --decode)
            )
        )
        (@subcommand affine =>
            (@group action +required =>
                (@arg encode: -e --encode)
                (@arg decode: -d --decode)
            )
            (@arg a: +required +takes_value "Slope")
            (@arg b: +required +takes_value "Intercept")
        )
        (@subcommand base64 =>
            (@group action +required =>
                (@arg encode: -e --encode)
                (@arg decode: -d --decode)
            )
        )
        (@subcommand caesar =>
            (@group action +required =>
                (@arg encode: -e --encode)
                (@arg decode: -d --decode)
            )
            (@arg shift: -s --shift +required +takes_value "Number of letter to shift")
        )
        (@subcommand rot13 =>
            (@group action +required =>
                (@arg encode: -e --encode)
                (@arg decode: -d --decode)
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
            print!("{}", ciphers::atbash::encode(buffer.borrow()))
        }
        ("rot13", _) => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => {}
                Err(err) => eprintln!("Cannot read string: {}", err)
            }
            print!("{}", ciphers::rot13::encode(buffer.borrow()))
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
            let (encode, decode) = (
                matches.is_present("encode"),
                matches.is_present("decode")
            );
            match (encode, decode) {
                (true, _) => {
                    match ciphers::caesar::encode(buffer.borrow(), shift) {
                        Ok(string) => print!("{}", string),
                        Err(err) => {
                            eprintln!("{}", err);
                            print!("{}", buffer)
                        }
                    }
                }
                (_, true) => {
                    match ciphers::caesar::decode(buffer.borrow(), shift) {
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
        ("base64", base64_matches) => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => {}
                Err(err) => eprintln!("Cannot read string: {}", err)
            }
            let matches = if let Some(matches) = base64_matches {
                matches
            } else {
                return;
            };
            let (encode, decode) = (
                matches.is_present("encode"),
                matches.is_present("decode")
            );
            match (encode, decode) {
                (true, _) => print!("{}", ciphers::base64::encode(buffer.borrow())),
                (_, true) => print!("{}", ciphers::base64::decode(buffer.borrow())),
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
            let (encode, decode) = (
                matches.is_present("encode"),
                matches.is_present("decode")
            );
            match (encode, decode) {
                (true, _) => {
                    match ciphers::affine::encode(buffer.borrow(), a, b) {
                        Ok(string) => print!("{}", string),
                        Err(err) => {
                            eprintln!("{}", err);
                            print!("{}", buffer)
                        }
                    }
                }
                (_, true) => {
                    match ciphers::affine::decode(buffer.borrow(), a, b) {
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
