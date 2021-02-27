#[macro_use]
extern crate clap;
extern crate clipboard;
extern crate termion;

use std::iter;
use std::convert::TryInto;
use clap::{Arg, App};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use rand::{Rng, thread_rng};
use rand::distributions::{Distribution};
use termion::{color};

const CHARSET_NUMBER: &[u8] = b"0123456789";
const CHARSET_LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const CHARSET_UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const CHARSET_SYMBOL: &[u8] = b" !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

struct CharacterDistributor {
    characters: Vec<u8>
}

impl Distribution<u8> for CharacterDistributor {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        let range: u32 = self.characters.len().try_into().unwrap();

        loop {
            let var = rng.next_u32() >> (32 - 6);
            if var < range {
                return self.characters[var as usize];
            }
        }
    }
}

fn main() {
    let matches = App::new("pw")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Generates a random password with the given limitations, and copies to the clipboard.")
        .arg(Arg::with_name("numeric")
            .long("numeric")
            .help("Prohibit numeric characters."))
        .arg(Arg::with_name("symbol")
            .long("symbol")
            .help("Prohibit symbol characters."))
        .arg(Arg::with_name("alpha")
            .long("alpha")
            .help("Prohibit all alphabetic characters."))
        .arg(Arg::with_name("lower")
            .long("lower")
            .help("Prohibit lowercase alphabetic characters."))
        .arg(Arg::with_name("upper")
            .short("u")
            .long("upper")
            .help("Prohibit uppercase alphabetic characters."))
        .arg(Arg::with_name("diceware")
            .long("diceware")
            .conflicts_with_all(&["numeric", "symbol", "alpha", "lower", "upper"])
            .help("Diceware passphrase (exclusive to other options)."))
        .arg(Arg::with_name("length")
            .long("length")
            .takes_value(true)
            .help("The desired length of the password."))
        .get_matches();

    let numeric: bool = matches.occurrences_of("numeric") > 0;
    let alpha: bool = matches.occurrences_of("alpha") > 0;
    let symbol: bool = matches.occurrences_of("symbol") > 0;
    let upper: bool = alpha || matches.occurrences_of("upper") > 0;
    let lower: bool = alpha || matches.occurrences_of("lower") > 0;
    let diceware: bool = matches.occurrences_of("diceware") > 0;
    let length: u32 = matches.value_of("length").unwrap_or(if diceware { "4" } else { "16" }).parse().unwrap();

    if diceware {
        panic!("not implemented");
    }

    let charset: Vec<u8>;

    match (lower, upper, numeric, symbol) {
        (false, true, true, true) => charset = CHARSET_LOWER.to_vec(),
        (true, false, true, true) => charset = CHARSET_UPPER.to_vec(),
        (true, true, false, true) => charset = CHARSET_NUMBER.to_vec(),
        (true, true, true, false) => charset = CHARSET_SYMBOL.to_vec(),
        (false, false, true, true) => charset = [CHARSET_LOWER, CHARSET_UPPER].concat(),
        (false, true, false, true) => charset = [CHARSET_LOWER, CHARSET_NUMBER].concat(),
        (false, true, true, false) => charset = [CHARSET_LOWER, CHARSET_SYMBOL].concat(),
        (true, false, true, false) => charset = [CHARSET_UPPER, CHARSET_SYMBOL].concat(),
        (true, false, false, true) => charset = [CHARSET_UPPER, CHARSET_NUMBER].concat(),
        (true, true, false, false) => charset = [CHARSET_NUMBER, CHARSET_SYMBOL].concat(),
        (false, false, false, true) => charset = [CHARSET_UPPER, CHARSET_LOWER, CHARSET_NUMBER].concat(),
        (false, false, true, false) => charset = [CHARSET_UPPER, CHARSET_LOWER, CHARSET_SYMBOL].concat(),
        (false, true, false, false) => charset = [CHARSET_LOWER, CHARSET_NUMBER, CHARSET_SYMBOL].concat(),
        (true, false, false, false) => charset = [CHARSET_UPPER, CHARSET_NUMBER, CHARSET_SYMBOL].concat(),
        (false, false, false, false) => charset = [CHARSET_UPPER, CHARSET_LOWER, CHARSET_NUMBER, CHARSET_SYMBOL].concat(),
        (true, true, true, true) => panic!("At least one character type must be allowed."),
    }

    let mut rng = thread_rng();

    let distributor = CharacterDistributor {
        characters: charset.to_vec()
    };

    let chars: String = iter::repeat(())
        .map(|()| rng.sample(&distributor))
        .map(char::from)
        .take(length as usize)
        .collect();

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(chars.to_owned()).unwrap();

    println!("{}Password copied to clipboard.", color::Fg(color::Green));
}
