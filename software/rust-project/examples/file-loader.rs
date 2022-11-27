use std::{fs, path::PathBuf};

use clap::{arg, command, value_parser};

fn main() {
    let matches = command!()
        .arg(
            arg!([filename] "Filename to load from.")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let filename = matches
        .get_one::<PathBuf>("filename")
        .expect("'filename' is required"); // use expect because clap matcher should have failed already

    println!("Filename: {}", filename.display());

    // there is no way to read the file into memory and overflow without an explicit `unsafe` block
    // which would be useless anyways, because reading a file into memory has safe ways to do it already in rust
    // EVERYTHING is bounds checked, and you CANNOT break those rules without `unsafe` blocks.
    // and any errors that could occur on load, HAVE to be handled, otherwise the program will "panic" and quit.
    if let Ok(content) = fs::read_to_string(filename) {
        println!("\n\r-- File Contents -- \n\r{content}");
    } else {
        println!("Could not find or load file contents.");
    }
}
