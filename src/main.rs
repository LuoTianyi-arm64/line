use std::env;
use std::fs::File;
use std::io::Read;
use line::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let mut file = File::open(&args[1]).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let header = get_header_info(&buffer);
        println!("{header:#?}");
    }
}
