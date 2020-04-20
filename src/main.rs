#[macro_use]
extern crate clap;

use std::{
    io::{
        self, Read,
    },
    borrow::Borrow,
};

mod atbash;
mod rot13;
mod caesar;
mod tests;

fn main() {
    let matches = clap_app!(crypt =>
        (version: "0.1.0")
        (author: "dikuchan <dikuchan@yahoo.com>")
        (about: "Cipher piped strings")
        (@subcommand atbash =>
            (@group action +required =>
                (@arg encrypt: -e --encrypt)
                (@arg decrypt: -d --decrypt)
            )
        )
        (@subcommand rot13 =>
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
            (@arg shift: -s --shift +required +takes_value "Number of letter to shift")
        )
    ).get_matches();

    match matches.subcommand() {
        ("atbash", _) => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => {}
                Err(err) => eprintln!("Cannot read string: {}", err)
            }
            match atbash::encrypt(buffer.borrow()) {
                Some(string) => print!("{}", string),
                None => print!("{}", buffer)
            }
        }
        ("rot13", _) => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => {}
                Err(err) => eprintln!("Cannot read string: {}", err)
            }
            match rot13::encrypt(buffer.borrow()) {
                Some(string) => print!("{}", string),
                None => print!("{}", buffer)
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
                print!("{}", buffer);
                return;
            };
            let shift = if let Some(shift) = matches.value_of("shift") {
                shift
            } else {
                print!("{}", buffer);
                return;
            };
            let shift = match shift.parse::<u8>() {
                Ok(num) => num,
                Err(err) => {
                    eprintln!("Erroneous shift: {}", err);
                    print!("{}", buffer);
                    return;
                }
            };
            let (encrypt, decrypt) = (
                matches.is_present("encrypt"),
                matches.is_present("decrypt")
            );
            match (encrypt, decrypt) {
                (true, _) => {
                    match caesar::encrypt(buffer.borrow(), shift) {
                        Some(string) => print!("{}", string),
                        None => print!("{}", buffer)
                    }
                }
                (_, true) => {
                    match caesar::decrypt(buffer.borrow(), shift) {
                        Some(string) => print!("{}", string),
                        None => print!("{}", buffer)
                    }
                }
                _ => unreachable!()
            }
        }
        ("", None) => { return; }
        _ => unreachable!()
    }
}
