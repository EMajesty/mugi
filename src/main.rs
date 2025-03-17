use std::{fs::File, process::exit};
// use std::io::prelude::*;
use std::path::Path;
use std::{env, fs};
use std::io;

use termion::{raw::IntoRawMode, color};

use std::io::{stdin, stdout, Read, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut path = "";

    let mut document = String::new();

    match args.len() {
        0 => path = "",
        1 => path = "",
        2 => {
            path = &args[1];
            document = fs::read_to_string(path).unwrap();
        },
        _ => return Ok(()),
    }

    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), document).unwrap();
    stdout.flush().unwrap();

    let mut input = stdin().bytes();
    loop {
        if let Some(Ok(byte)) = input.next() {
            match byte {
                // CTRL + C terminates the program
                3 => return Ok(()),
                // Enter
                13 => {
                    document.push(13 as char);
                    document.push(10 as char);
                    write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), document).unwrap();
                    stdout.flush().unwrap();
                }
                // CTRL + S saves the file
                19 => {
                    let mut file = File::create(&path).unwrap();
                    file.write_all(document.as_bytes()).unwrap();
                }
                // Backspace
                127 => {
                    let popped = document.pop();
                    if popped.unwrap() == 10 as char {
                        document.pop();
                    }
                    write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), document).unwrap();
                    stdout.flush().unwrap();
                }
                _ => {
                    document.push(byte as char);
                    write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), document).unwrap();
                    stdout.flush().unwrap();
                }
            }
        }
    }
}
