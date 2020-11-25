#[macro_use]
extern crate clap;

use crate::{
    error::CipherError as Error,
    ciphers::cipher::*,
    ciphers::{
        affine::Affine,
        atbash::Atbash,
        bacon::Bacon,
        caesar::Caesar,
        rot13::Rot13,
    },
};

use std::io::{self, Read};

mod ciphers;
mod error;
mod utils;

macro_rules! assign {
    ($cipher:ident, $matches:ident) => {
    match ($matches.is_present("encrypt"), $matches.is_present("decrypt")) {
        (true, _) => $cipher.encrypt(&parse_buffer($matches)?)?,
        (_, true) => $cipher.decrypt(&parse_buffer($matches)?)?,
        _ => unreachable!()
    }
    };
}

fn parse_buffer(matches: &clap::ArgMatches) -> Result<String, io::Error> {
    let mut buffer = String::new();
    match matches.value_of("input") {
        Some(input) => buffer = input.to_string(),
        None => { io::stdin().read_to_string(&mut buffer)?; }
    };

    Ok(buffer)
}

fn main() -> Result<(), Error> {
    let matches = clap_app!(crypt =>
        (version: "0.1.0")
        (author: "dikuchan <dikuchan@yahoo.com>")
        (about: "Simple command line encryption tool")
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
            (@arg key: +required +takes_value "Word to encode in text")
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
            let matches = affine_matches.unwrap();
            let slope = matches.value_of("slope").unwrap().parse::<i64>()?;
            let intercept = matches.value_of("intercept").unwrap().parse::<i64>()?;
            let cipher = Affine::new(slope, intercept)?;
            assign!(cipher, matches)
        }
        ("atbash", atbash_matches) => {
            let matches = atbash_matches.unwrap();
            let cipher = Atbash::new();
            assign!(cipher, matches)
        }
        ("bacon", bacon_matches) => {
            let matches = bacon_matches.unwrap();
            let key = matches.value_of("key").unwrap();
            let cipher = Bacon::new(key);
            assign!(cipher, matches)
        }
        ("caesar", caesar_matches) => {
            let matches = caesar_matches.unwrap();
            let shift = matches.value_of("shift").unwrap().parse::<u8>()?;
            let cipher = Caesar::new(shift)?;
            assign!(cipher, matches)
        }
        ("rot13", rot13_matches) => {
            let matches = rot13_matches.unwrap();
            let cipher = Rot13::new();
            assign!(cipher, matches)
        }
        ("", None) => return Ok(()),
        _ => unreachable!(),
    };

    print!("{}", result);

    Ok(())
}
