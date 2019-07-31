extern crate rand;

use rand::Rng;
use std::io;
use std::time::Instant;

struct StatData {
    times: Vec<u128>,
    tries: Vec<u32>
}

impl StatData {
    fn print(&self) {
    let times: Vec<f64> = self.times.iter().cloned().map(|x| x as f64 / 1000.0).collect();

    println!(
        "Fertig!\nZeitinformation\n
        μ: {}s\n
        max: {}s\n
        min: {}s",
        mean_f64(&times),
        times.iter().cloned().fold(0. / 0., f64::max),
        times.iter().cloned().fold(1. / 0., f64::min)
    );
    println!("Versuchsinformation\n
            μ: {}\n
            max: {}\n
            min: {}\n",
            mean_u32(&self.tries),
            self.tries.iter().cloned().fold(u32::min_value(), u32::max),
            self.tries.iter().cloned().fold(u32::max_value(), u32::min));
    }
}

fn main() {
    println!("Minimum (oder 10): ");
    let min = read_num(Some(10));
    println!("Maximum (nicht inklusive oder 100): ");
    let max = read_num(Some(100));
    println!("Anzahl (oder 5): ");
    let cnt = read_num(Some(5));

    game(min, max, cnt);
}

fn game(min: u32, max: u32, rounds: u32) {
    let mut stats = StatData {
        times: Vec::new(),
        tries: Vec::new(),
    };
    for _ in 0..rounds {
        let val1 = rand::thread_rng().gen_range(min, max);
        let val2 = rand::thread_rng().gen_range(min, max);
        let res = val1 * val2;
        println!("{}*{}=", val1, val2);
        let start = Instant::now();
        let mut tries = 0;
        loop {
            tries += 1;
            if read_num(None) == res {
                break;
            }
        }
        stats.tries.push(tries);
        stats.times.push(start.elapsed().as_millis());
    }

    stats.print();
}

fn read_num(default: Option<u32>) -> u32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");

    match default {
        Some(default) => input.trim().parse().unwrap_or(default),
        None => {
            match input.trim().parse::<u32>() {
                Ok(val) => val,
                Err(_)  => read_num(None)
            }
        }
    }
}

fn mean_f64(list: &Vec<f64>) -> f64 {
    let sum: f64 = Iterator::sum(list.iter());
    f64::from(sum) / (list.len() as f64)
}

fn mean_u32(list: &Vec<u32>) -> f64 {
    let sum: u32 = Iterator::sum(list.iter());
    f64::from(sum) / (list.len() as f64)
}
