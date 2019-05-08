extern crate clap;
use clap::{App, Arg};
use std::io;

fn main() {
    let matches = App::new("Vigenere solver")
        .version("0.1")
        .author(clap::crate_authors!("\n"))
        .arg(
            Arg::with_name("decode")
                .help("Specify to decode a string")
                .takes_value(true)
                .value_name("cipher_text")
                .short("d")
                .long("decode"),
        )
        .arg(
            Arg::with_name("encode")
                .help("Specify to encode a string")
                .takes_value(true)
                .value_name("plain_text")
                .short("e")
                .long("encode"),
        )
        .get_matches();

    if matches.is_present("decode") {
        let cipher = matches.value_of("decode").unwrap().to_uppercase();
        decoder(cipher);
    } else if matches.is_present("encode") {
        let plain = matches.value_of("encode").unwrap().to_uppercase();
        encoder(plain);
    } else {
        println!("You need to supply either of\n-e PLAINTEXT\n-d CIPHERTEXT");
        return;
    }
}

fn encoder(plain: String) {
    // get key and encrypt
    loop {
        // read key
        let mut key = String::new();
        io::stdin()
            .read_line(&mut key)
            .expect("Failed to read line!");
        let mut key = key.to_uppercase();
        // remove trailing \n
        key.pop();
        let mut enciphered: Vec<u8> = Vec::new();
        // iterate over each character of plain text and key (cyclic)
        for (a, b) in plain.bytes().zip(key.bytes().cycle()) {
            // compute to decoded value and add it to the vector
            enciphered.push(encode(a, b));
        }
        println!("{}", std::str::from_utf8(&enciphered).unwrap());
    }
}

fn decoder(cipher: String) {
    // get key and decrypt
    loop {
        // read key
        let mut key = String::new();
        io::stdin()
            .read_line(&mut key)
            .expect("Failed to read line!");
        let mut key = key.to_uppercase();
        // remove trailing \n
        key.pop();
        let mut deciphered: Vec<u8> = Vec::new();
        // iterate over each character of cipher text and key (cyclic)
        for (a, b) in cipher.bytes().zip(key.bytes().cycle()) {
            // compute to decoded value and add it to the vector
            deciphered.push(decode(a, b));
        }
        println!("{}", std::str::from_utf8(&deciphered).unwrap());
    }
}

fn decode(cipher: u8, key: u8) -> u8 {
    let alpha = "A".as_bytes()[0];
    (26         // add 26 to avoid underflow
    + cipher    // the original encrypted character
    - key       // subtract the key character
    - 1)        // subtract 1 because a=1 not 0
        % 26    // wrap around to remain in [0,25]
        + alpha // add "A" to get back into ascii [65,90]
}

fn encode(plain: u8, key: u8) -> u8 {
    let alpha = "A".as_bytes()[0];
    (plain          // the original char to encrypt
     + key          // add the key
     - 2 * alpha    // subtract to get in remove ascii part
     + 1)           // add 1 because a=1 not 0
    % 26            // wrap around to remain in [0,25]
    + alpha // add "A" to get back in ascii [65,90]
}
