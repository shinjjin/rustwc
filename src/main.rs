use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    readout(&args[1]);
}

fn readout(filename: &str) {
    let mut file = match File::open(filename) {
        Err(why) => {
            println!("couldn open file: {}", why);
            return;
        }
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => println!("couldnt read file: {}", why),
        Ok(bytes) => println!(
            "lines: {}, words: {}, chars: {}, bytes: {}, path: {}",
            s.lines().count(),
            s.split_whitespace().count(),
            s.chars().count(),
            bytes,
            filename
        ),
    };
}
