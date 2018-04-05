use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, BufReader};
use std::str::FromStr;

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
    fn read_width_option(arg: &String) -> Result<usize, <usize as FromStr>::Err> {
        arg.chars().skip(1).collect::<String>().parse::<usize>()
    }

    fn should_read_stdin(args: &Vec<String>) -> Option<usize> {
        if args.is_empty() {
            Some(DEFAULT_MAX_WIDTH)
        } else if args.len() == 1 {
            read_width_option(args.first().unwrap()).ok()
        } else {
            None
        }
    }

    let mut max_width = match term_size::dimensions() {
        Some((w, _)) => w,
        _ => DEFAULT_MAX_WIDTH,
    };

    let args = {
        let mut args: Vec<_> = env::args().collect();
        args.remove(0);
        args
    };

    let mut errs: Vec<Box<Error>> = Vec::new();

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
            if let Ok(nmw) = read_width_option(&arg) {
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
