extern crate rand;

use rand::Rng;
use std::io;
use std::time::Instant;

fn main() {
    let mut min = String::new();
    let mut max = String::new();
    let mut cnt = String::new();
    println!("Minimum (oder 10): ");
    io::stdin()
        .read_line(&mut min)
        .expect("Failed to read line");
    let min: u32 = min.trim().parse().unwrap_or(10);
    println!("Maximum (nicht inklusive oder 100): ");
    io::stdin()
        .read_line(&mut max)
        .expect("Failed to read line");
    let max: u32 = max.trim().parse().unwrap_or(100);
    println!("Anzahl (oder 5): ");
    io::stdin()
        .read_line(&mut cnt)
        .expect("Failed to read line");
    let cnt: u32 = cnt.trim().parse().unwrap_or(5);

    game(min, max, cnt);
}

fn game(min: u32, max: u32, rounds: u32) {
    let mut times = Vec::new();

    for _ in 0..rounds {
        let val1 = rand::thread_rng().gen_range(min, max);
        let val2 = rand::thread_rng().gen_range(min, max);
        let res = val1 * val2;
        let mut guess;
        println!("{}*{}=", val1, val2);
        let start = Instant::now();
        loop {
            guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read guess");
            let guess: u32 = guess.trim().parse().unwrap_or(0);

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
