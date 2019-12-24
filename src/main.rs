use checkpoint as cp;
use std::env;
use std::io;
use std::process::exit;

#[macro_use]
mod util;
mod checkpoint;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let (Some(opt), Some(val)) = (args.get(1), args.get(2)) {
        arg_handler(opt, val);
    } else {
        wprintln!("* (1)");
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            wprintln!("* Ich erwarte eine Eingabe");
            exit(1);
        }
        util::trim_newline(&mut input);
        if !input.to_ascii_lowercase().as_str().eq("erstderanfang") {
            wprintln!("* Ich erwarte eine andere Eingabe");
            exit(1);
        } else {
            input_handler();
        }
    }
}

fn arg_handler(opt: &str, val: &str) {
    if let "CP" = opt {
        wprintln!("Du willst wohl den Checkpoint: {} einlösen", val);
        checkpoint_handler(val)
    }
}

fn input_handler() {
    wprintln!("Sehr gut, weter so. Als");
    wprintln!("nechstes: Du weißt doch wi");
    wprintln!("Checkpoints in alten Spielen funkioniert");
    wprintln!("haben, oder? Also mache ic");
    wprintln!("jetzt das gleche mit dir.");
    wprintln!("Wenn du ds Programm manuell");
    wprintln!("von CMD au diret startest,");
    wprintln!("dann kanst du das genau");
    wprintln!("so machn: '<programm-name> CP <check-point-wert>'.");
    wprintln!("Dami du das ganze gleich");
    wprintln!("ausprobiern kannst, bekommst du den");
    wprintln!("checkpoint: '1' (ohne Anfuhrungszeichen), ich");
    wprintln!("wunsche dir viel Erfolg dabei.");
}

fn checkpoint_handler(val: &str) {
    match val {
        "1" => cp::cp1(),
        "23" => cp::cp23(),
        "44" => cp::cp44(),
        "474127" => cp::cp474127(),
        _ => {
            wprintln!("* Leider hast du falsch geraten. Es ist nicht so einfach, wie es ausschaut.")
        }
    }
}
