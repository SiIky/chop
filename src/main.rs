use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, BufReader};

extern crate term_size;

extern crate unicode_width;
use unicode_width::UnicodeWidthStr;

const DEFAULT_MAX_WIDTH: usize = 80;

trait ChopWidth {
    fn chop_width(self, max_width: usize) -> String;
}

impl ChopWidth for String {
    fn chop_width(self, max_width: usize) -> String {
        let mut cw = max_width;
        let mut line = self;

        while line.width_cjk() > max_width {
            cw -= 1;
            // TODO: Improve this, prob some `drop()`, `pop()` or similar `Chars` method
            line = line.chars().take(cw).collect();
        }

        line.to_string()
    }
}

fn real_main() -> Result<(), Vec<Box<Error>>> {
    fn read_width_option(arg: &String) -> Option<usize> {
        if arg.is_empty() || arg.chars().nth(0).unwrap() != '-' {
            return None;
        }

        let arg = arg.chars().skip(1).collect::<String>();
        if arg.chars().all(|c| c.is_ascii_digit()) {
            arg.parse::<usize>().ok()
        } else {
            None
        }
    }

    fn should_read_stdin(args: &Vec<String>) -> Option<usize> {
        if args.is_empty() {
            /* no arguments given */
            Some(term_size::dimensions().map_or(DEFAULT_MAX_WIDTH, |p| p.0))
        } else if args.len() == 1 {
            /* a single argument was given, try to parse a width */
            read_width_option(args.first().unwrap())
        } else {
            /* more than one argument was given, don't read from stdin */
            None
        }
    }

    let args = {
        let mut args: Vec<_> = env::args().collect();
        args.remove(0);
        args
    };

    let mut errs: Vec<Box<Error>> = Vec::new();
    let mut max_width = DEFAULT_MAX_WIDTH;

    if let Some(nmw) = should_read_stdin(&args) {
        max_width = nmw;

        /* read stdin */
        let stdin = stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(line) => println!("{}", line.chop_width(max_width)),
                Err(e) => errs.push(Box::new(e)),
            }
        }
    } else {
        /* treat arguments as filenames and try to read them */
        for arg in args {
            if let Some(nmw) = read_width_option(&arg) {
                max_width = nmw;
                continue;
            }

            let reader = match File::open(arg) {
                Ok(reader) => BufReader::new(reader),
                Err(e) => {
                    errs.push(Box::new(e));
                    continue;
                }
            };

            for line in reader.lines() {
                match line {
                    Ok(line) => println!("{}", line.chop_width(max_width)),
                    Err(e) => errs.push(Box::new(e)),
                }
            }
        }
    }

    if errs.is_empty() {
        Ok(())
    } else {
        Err(errs)
    }
}

fn main() {
    if let Err(ve) = real_main() {
        for e in ve {
            eprintln!("{}", e);
        }
    }
}
