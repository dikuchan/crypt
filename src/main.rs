#[macro_use]
extern crate clap;

use std::{
    io::{
        self, Read,
    },
};

mod ciphers;
mod utils;

fn parse_buffer(matches: &clap::ArgMatches) -> Option<String> {
    let mut buffer = String::new();
    match matches.value_of("input") {
        Some(input) => buffer = input.to_string(),
        None => {
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => {}
                Err(_) => return None
            }
        }
    };

    Some(buffer)
}

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
            (@arg input: +takes_value "String to encode")
        )
        (@subcommand affine =>
            (@group action +required =>
                (@arg encrypt: -e --encrypt)
                (@arg decrypt: -d --decrypt)
            )
            (@arg slope: +required +takes_value "Slope")
            (@arg intercept: +required +takes_value "Intercept")
            (@arg input: +takes_value "String to decode")
        )
        (@subcommand bacon =>
            (@group action +required =>
                (@arg encrypt: -e --encrypt)
                (@arg decrypt: -d --decrypt)
            )
            (@arg input: +takes_value "String to decode")
        )
        (@subcommand caesar =>
            (@group action +required =>
                (@arg encrypt: -e --encrypt)
                (@arg decrypt: -d --decrypt)
            )
            (@arg shift: +required +takes_value "Number of letter to shift")
            (@arg input: +takes_value "String to decode")
        )
        (@subcommand rot13 =>
            (@group action +required =>
                (@arg encrypt: -e --encrypt)
                (@arg decrypt: -d --decrypt)
            )
            (@arg input: +takes_value "String to encode")
        )
    ).get_matches();

    match matches.subcommand() {
        ("affine", affine_matches) => {
            let matches = if let Some(matches) = affine_matches { matches } else { return; };
            let slope = matches.value_of("slope").unwrap();
            let slope = match slope.parse::<i64>() {
                Ok(num) => num,
                Err(err) => {
                    eprint!("Erroneous slope value: {}", err);
                    return;
                }
            };
            let intercept = matches.value_of("intercept").unwrap();
            let intercept = match intercept.parse::<i64>() {
                Ok(num) => num,
                Err(err) => {
                    eprint!("Erroneous intercept value: {}", err);
                    return;
                }
            };
            let (encrypt, decrypt) = (
                matches.is_present("encrypt"),
                matches.is_present("decrypt")
            );
            match (encrypt, decrypt) {
                (true, _) => {
                    match parse_buffer(matches) {
                        Some(buffer) => {
                            match ciphers::affine::encrypt(&buffer, slope, intercept) {
                                Ok(string) => print!("{}", string),
                                Err(err) => eprint!("{}", err)
                            }
                        },
                        None => eprint!("Nothing to encrypt")
                    }
                }
                (_, true) => {
                    match parse_buffer(matches) {
                        Some(buffer) => {
                            match ciphers::affine::decrypt(&buffer, slope, intercept) {
                                Ok(string) => print!("{}", string),
                                Err(err) => eprint!("{}", err)
                            }
                        },
                        None => eprint!("Nothing to decrypt")
                    }
                }
                _ => unreachable!()
            }
        }
        ("atbash", atbash_matches) => {
            let matches = if let Some(matches) = atbash_matches { matches } else { return; };
            match parse_buffer(matches) {
                Some(buffer) => print!("{}", ciphers::atbash::encrypt(&buffer)),
                None => eprint!("Nothing to encrypt")
            };
        }
        ("bacon", bacon_matches) => {
            let matches = if let Some(matches) = bacon_matches { matches } else { return; };
            let (encrypt, decrypt) = (
                matches.is_present("encrypt"),
                matches.is_present("decrypt")
            );
            match (encrypt, decrypt) {
                (true, _) => {
                    match parse_buffer(matches) {
                        Some(buffer) => match ciphers::bacon::encrypt(&buffer) {
                            Ok(result) => print!("{}", result),
                            Err(err) => eprint!("{}", err)
                        },
                        None => eprint!("Nothing to encrypt")
                    }
                }
                (_, true) => {
                    match parse_buffer(matches) {
                        Some(buffer) => match ciphers::bacon::decrypt(&buffer) {
                            Ok(result) => print!("{}", result),
                            Err(err) => eprintln!("{}", err)
                        },
                        None => eprint!("Nothing to decrypt")
                    }
                }
                _ => unreachable!()
            }
        }
        ("caesar", caesar_matches) => {
            let matches = if let Some(matches) = caesar_matches { matches } else { return; };
            let shift = matches.value_of("shift").unwrap();
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
                    match parse_buffer(matches) {
                        Some(buffer) => match ciphers::caesar::encrypt(&buffer, shift) {
                            Ok(result) => print!("{}", result),
                            Err(err) => eprintln!("{}", err)
                        },
                        None => eprint!("Nothing to encrypt")
                    }
                }
                (_, true) => {
                    match parse_buffer(matches) {
                        Some(buffer) => match ciphers::caesar::decrypt(&buffer, shift) {
                            Ok(result) => print!("{}", result),
                            Err(err) => eprintln!("{}", err)
                        },
                        None => eprint!("Nothing to decrypt")
                    }
                }
                _ => unreachable!()
            }
        }
        ("rot13", rot13_matches) => {
            let matches = if let Some(matches) = rot13_matches { matches } else { return; };
            match parse_buffer(matches) {
                Some(buffer) => print!("{}", ciphers::rot13::encrypt(&buffer)),
                None => eprint!("Nothing to encode")
            };
        }
        ("", None) => return,
        _ => unreachable!()
    }
}
