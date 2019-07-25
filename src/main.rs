extern crate rand;

use rand::Rng;
use std::io;
use std::time::Instant;

fn main() {
    let mut min = String::new();
    let mut max = String::new();
    let mut cnt = String::new();
    println!("Minimum: ");
    io::stdin()
        .read_line(&mut min)
        .expect("Failed to read line");
    let min: u32 = min.trim().parse().expect("Failed to parse");
    println!("Maximum (nicht inklusive): ");
    io::stdin()
        .read_line(&mut max)
        .expect("Failed to read line");
    let max: u32 = max.trim().parse().expect("Failed to parse");
    println!("Anzahl: ");
    io::stdin()
        .read_line(&mut cnt)
        .expect("Failed to read line");
    let cnt: u32 = cnt.trim().parse().expect("Failed to parse");

    game(min, max, cnt);
}

fn game(min: u32, max: u32, rounds: u32) {
    let mut times = Vec::new();

    for _ in 0..rounds {
        let val1 = rand::thread_rng().gen_range(min, max);
        let val2 = rand::thread_rng().gen_range(min, max);
        let res = val1 * val2;
        let mut guess = String::new();
        println!("{}*{}=", val1, val2);
        let start = Instant::now();
        loop {
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read guess");
            let guess: u32 = guess.trim().parse().expect("Failed to parse guess");

            if guess == res {
                break;
            }
        }
        times.push(start.elapsed().as_millis());
    }

    let times: Vec<f64> = times.iter().cloned().map(|x| x as f64 / 1000.0).collect();

    println!(
        "Fertig!\n
             μ: {}s\n
             max: {}s\n
             min: {}s",
        times.iter().cloned().fold(0.0, |a, b| { a + b }) / rounds as f64,
        times.iter().cloned().fold(0. / 0., f64::max),
        times.iter().cloned().fold(1. / 0., f64::min)
    );
}
