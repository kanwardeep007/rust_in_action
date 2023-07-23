use clap::{App, Arg};
use regex::Regex;
use std::io;
use std::io::BufRead;
use std::io::{Read, Result};
use std::{fs::File, io::BufReader};

enum Lines<'a> {
    File(BufReader<File>),
    Stdin(std::io::StdinLock<'a>),
}

impl<'a> BufRead for Lines<'a> {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        match self {
            Lines::File(inner) => inner.fill_buf(),
            Lines::Stdin(inner) => inner.fill_buf(),
        }
    }
    fn consume(&mut self, amt: usize) {
        match self {
            Lines::File(inner) => inner.consume(amt),
            Lines::Stdin(inner) => inner.consume(amt),
        }
    }
}

impl<'a> Read for Lines<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            Lines::File(inner) => inner.read(buf),
            Lines::Stdin(inner) => inner.read(buf),
        }
    }
}

fn process_lines<T: BufRead>(reader: T, re: Regex) {
    for (i, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{i} :{}", line),
            None => (),
        }
    }
}
fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("path")
                .help("Path of the file to search for")
                .takes_value(true)
                .required(false),
        )
        .get_matches();
    let pattern = Regex::new(args.value_of("pattern").unwrap()).unwrap();
    let file_path = args.value_of("path");
    let xx = match file_path {
        Some(inner) => {
            let file = File::open(inner).unwrap();
            let buf_reader = BufReader::new(file);
            Lines::File(buf_reader)
            // process_lines(buf_reader, pattern)
        }
        None => {
            let stdin = io::stdin();
            let handle = stdin.lock();
            Lines::Stdin(handle)
            // process_lines(handle, pattern)
        }
    };
    process_lines(xx, pattern);
}
