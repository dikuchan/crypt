#[macro_use]
extern crate clap;

use std::{
    io::{self, Read},
    error,
};

mod ciphers;
mod utils;

fn parse_buffer(matches: &clap::ArgMatches) -> Result<String, Box<dyn error::Error>> {
    let mut buffer = String::new();
    match matches.value_of("input") {
        Some(input) => buffer = input.to_string(),
        None => { let _ = io::stdin().read_to_string(&mut buffer)?; }
    };

    Ok(buffer)
}

fn main() -> Result<(), Box<dyn error::Error>> {
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

    let result = match matches.subcommand() {
        ("affine", affine_matches) => {
            let matches = if let Some(matches) = affine_matches { matches } else { return Ok(()); };
            let slope = matches.value_of("slope").unwrap().parse::<i64>()?;
            let intercept = matches.value_of("intercept").unwrap().parse::<i64>()?;
            let (encrypt, decrypt) = (
                matches.is_present("encrypt"),
                matches.is_present("decrypt")
            );
            match (encrypt, decrypt) {
                (true, _) => ciphers::affine::encrypt(&parse_buffer(matches)?, slope, intercept)?,
                (_, true) => ciphers::affine::decrypt(&parse_buffer(matches)?, slope, intercept)?,
                _ => unreachable!()
            }
        }
        ("atbash", atbash_matches) => {
            let matches = if let Some(matches) = atbash_matches { matches } else { return Ok(()); };
            let buffer = parse_buffer(matches)?;
            ciphers::atbash::encrypt(&buffer)
        }
        ("bacon", bacon_matches) => {
            let matches = if let Some(matches) = bacon_matches { matches } else { return Ok(()); };
            let (encrypt, decrypt) = (
                matches.is_present("encrypt"),
                matches.is_present("decrypt")
            );
            match (encrypt, decrypt) {
                (true, _) => ciphers::bacon::encrypt(&parse_buffer(matches)?)?,
                (_, true) => ciphers::bacon::decrypt(&parse_buffer(matches)?)?,
                _ => unreachable!()
            }
        }
        ("caesar", caesar_matches) => {
            let matches = if let Some(matches) = caesar_matches { matches } else { return Ok(()); };
            let shift = matches.value_of("shift").unwrap().parse::<u8>()?;
            let (encrypt, decrypt) = (
                matches.is_present("encrypt"),
                matches.is_present("decrypt")
            );
            match (encrypt, decrypt) {
                (true, _) => ciphers::caesar::encrypt(&parse_buffer(matches)?, shift)?,
                (_, true) => ciphers::caesar::decrypt(&parse_buffer(matches)?, shift)?,
                _ => unreachable!()
            }
        }
        ("rot13", rot13_matches) => {
            let matches = if let Some(matches) = rot13_matches { matches } else { return Ok(()); };
            ciphers::rot13::encrypt(&parse_buffer(matches)?)
        }
        ("", None) => return Ok(()),
        _ => unreachable!(),
    };

    print!("{}", result);

    Ok(())
}
