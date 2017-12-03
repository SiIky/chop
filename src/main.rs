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
            line = line.chars().take(cw).collect();
        }

        line.to_string()
    }
}

fn real_main() -> Result<(), Vec<Box<Error>>> {
    let max_width = if let Some((w, _)) = term_size::dimensions() {
        w
    } else {
        DEFAULT_MAX_WIDTH
    };

    let args = {
        let mut args: Vec<_> = env::args().collect();
        args.remove(0);
        args
    };

    let mut errs: Vec<Box<Error>> = Vec::new();

    if args.is_empty() {
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

    if errs.is_empty() { Ok(()) } else { Err(errs) }
}

fn main() {
    if let Err(ve) = real_main() {
        for e in ve {
            println!("{}", e);
        }
    }
}
