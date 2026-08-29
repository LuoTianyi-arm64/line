use line::*;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let mut file = File::open(&args[1]).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let header = get_header_info(&buffer).unwrap_or_else(|e| {
            eprintln!("{e:?}");
            std::process::exit(1);
        });
        println!("{header:#?}");
        let phdr = get_program_header_table(header, &buffer).unwrap_or_else(|e| {
            eprintln!("{e:?}");
            std::process::exit(1);
        });
        println!("{phdr:#?}");
    }
}
