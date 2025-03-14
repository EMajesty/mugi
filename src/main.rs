use termion::raw::IntoRawMode;

use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut document = String::new();

    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    let mut input = stdin().bytes();
    loop {
        if let Some(Ok(byte)) = input.next() {
            match byte {
                3 => break,
                13 => {
                    document.push(13 as char);
                    document.push(10 as char);
                    write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), document).unwrap();
                    stdout.flush().unwrap();
                }
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
