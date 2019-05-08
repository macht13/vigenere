#[macro_use]
extern crate text_io;

fn main() {
    println!("Do you want to (E)ncrypt or (D)ecrypt? [E/D]");
    let mode: String = read!("{}\n");
    match mode
        .replace("\r", "")
        .to_uppercase()
        .chars()
        .next()
        .expect("Invalid input supplied")
    {
        'E' => encoder(),
        'D' => decoder(),
        _ => println!("Invalid! Please enter either E or D"),
    };
}

fn encoder() {
    // get input
    println!("Please enter plaintext:");
    let mut plain: String = read!("{}\n");
    plain = plain.replace("\r", "").replace(" ", "").to_uppercase();
    println!("Now please enter any number of keys as single lines:");
    // get key and encrypt
    loop {
        // read key
        let mut key: String = read!("{}\n");
        key = key.replace("\r", "").replace(" ", "").to_uppercase();

        let mut enciphered: Vec<u8> = Vec::new();
        // iterate over each character of plain text and key (cyclic)
        for (a, b) in plain.bytes().zip(key.bytes().cycle()) {
            // compute to decoded value and add it to the vector
            enciphered.push(encode(a, b));
        }
        println!("Cipher: {}", std::str::from_utf8(&enciphered).unwrap());
    }
}

fn decoder() {
    // get input
    println!("Please enter ciphertext:");
    let mut cipher: String = read!("{}\n");
    cipher = cipher.replace("\r", "").replace(" ", "").to_uppercase();
    println!("Now please enter any number of keys as single lines:");
    // get key and decrypt
    loop {
        // read key
        let mut key: String = read!("{}\n");
        key = key.replace("\r", "").replace(" ", "").to_uppercase();

        let mut deciphered: Vec<u8> = Vec::new();
        // iterate over each character of cipher text and key (cyclic)
        for (a, b) in cipher.bytes().zip(key.bytes().cycle()) {
            // compute to decoded value and add it to the vector
            deciphered.push(decode(a, b));
        }
        println!("Plain: {}", std::str::from_utf8(&deciphered).unwrap());
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
