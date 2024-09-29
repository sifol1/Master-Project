mod odds;
mod math;

use std::fs::File;
use std::io::{self, prelude::*};
use std::env;
use regex::Regex;
use chrono::{Local, DateTime};
use odds::{orthoderivative, differential_spectrum, orthoderivative_with_basis};
use math::{get_primitive_polynomial, multiplication, square_and_multiply};

fn parse_line(line: &str, re: &Regex) -> Vec<(u32, u32)> {
    re.captures_iter(line)
        .map(|cap| {
            let coeff: u32 = cap[1].parse().unwrap();
            let exp: u32 = cap[2].parse().unwrap();
            (coeff, exp)
        })
        .collect()
}

fn format_asctime(time: DateTime<Local>) -> String {
    let naive_time = time.naive_local();
    let formatted_time = format!("{}", naive_time.format("%a %b %e %T %Y"));
    formatted_time
}

fn compute_func(func: &[(u32, u32)], x: u32, dimension: u32, primitive: u32) -> u32 {
    let mut sum = 0;
    for &(coeff, exp) in func {
        let compute_exp = square_and_multiply(x, exp, primitive, dimension);
        sum ^= multiplication(coeff, compute_exp, primitive, dimension);
    }
    sum
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let filename: &str = args.get(1).map_or("default.o", |s| s.as_str()); // "default.o" is default

    // Compile regex once
    let re = Regex::new(r"\((\d+),\s*(\d+)\)").unwrap(); 

    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();

    let mut _dimension = 0;

    // Process the first line
    if let Some(first_line) = lines.next() {
        let first_line = first_line?;
        _dimension = first_line.trim().parse().map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Error parsing dimension: {}", e))
        })?;
    } else {
        // Handle case where the file is empty
        println!("File is empty");
        return Ok(());
    }

    let primitive = get_primitive_polynomial(_dimension as usize);

    // Process the rest of the lines
    for line in lines {
        let line = line?;
        let func = parse_line(&line, &re);
        let tt: Vec<u32> = (0..(1 << _dimension))
            .map(|x| compute_func(&func, x, _dimension, primitive))
            .collect();
        let od = orthoderivative(_dimension, &tt, primitive);
        differential_spectrum(&od, _dimension);
        println!("");
    }

    Ok(())
}
     
