use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use clap::{Arg, command, value_parser};

mod huffman;

fn main() {
    let matches = command!()
        .arg(
            Arg::new("command")
                .required(true)
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("input_file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("output_file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    if let (Some(cmd), Some(input_filepath), Some(output_filepath)) = (
        matches.get_one::<String>("command"),
        matches.get_one::<PathBuf>("input_file"),
        matches.get_one::<PathBuf>("output_file"),
    ) {
        let mut input_file = File::open(input_filepath).unwrap();
        let mut output_file = File::create(output_filepath).unwrap();

        if cmd == "compress" {
            let mut content = String::new();
            input_file.read_to_string(&mut content).unwrap();

            let encoded = huffman::encode(&content, |s| s.chars().into_iter().collect());
            output_file.write(&encoded).unwrap();
        } else if cmd == "extract" {
            let content: Vec<u8> = fs::read(input_filepath).unwrap();

            let decoded = huffman::decode(&content, |t: Vec<char>| t.into_iter().collect());
            output_file.write(decoded.as_bytes()).unwrap();
        } else {
            panic!("Now a command!");
        }
    }
}
