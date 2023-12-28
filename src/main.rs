extern crate clap;

use clap::{Arg, Command};
use std::fs::File;
use std::io::{self,Read};

fn main() -> io::Result<()> {
    let matches = Command::new("Word Counter")
        .version("1.0")
        .author("Samuel Dasilva")
        .about("Counts the number of words in a text")
        .arg(
            Arg::new("file")
                .help("Sets the input text")
                .required(true)
                .index(1)
                .value_name("FILE"),
        )
        .get_matches();
    let file_name = matches.get_one::<String>("file").unwrap().to_string();
    let mut file = File::open(&file_name)?;
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer)?;

    let word_count = count_words(buffer);
    println!("{}", word_count);
    Ok(())
}

fn count_words(text: String) -> usize {
    text.split_whitespace().count()
}


