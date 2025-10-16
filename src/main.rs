use std::path::PathBuf;

use clap::{Arg, command, value_parser};

fn main() {
    let matches = command!()
        .arg(
            Arg::new("file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    if let Some(path) = matches.get_one::<PathBuf>("file") {
        println!("{:?}", path);
    }
}
