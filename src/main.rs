use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

struct DataLine {
    pass: String,
    lower_limit: i32,
    higher_limit: i32,
    letter: char
}


fn read_input_data() -> io::Result<Vec<DataLine>>{
    let f = File::open("inputData.txt")?;
    let f = BufReader::new(f);

    let mut vec:Vec<DataLine> = Vec::new();

    for line in f.lines() {
        let line_string = line.unwrap();
        let data_pieces:Vec<&str> = line_string.split(" ").collect();
        let limits:Vec<&str> = data_pieces[0].split("-").collect();
        let lower_limit:i32 = limits[0].parse::<i32>().unwrap();
        let higher_limit:i32 = limits[1].parse::<i32>().unwrap();
        let pass = String::from(data_pieces[2].clone());
        let letter = data_pieces[1].replace(":", "").chars().nth(0).unwrap();
        vec.push(DataLine{pass, lower_limit, higher_limit, letter});
    }

    Ok(vec)
}

fn simple_solution_part_1() -> i32 {
    let data = read_input_data().unwrap();
    let mut valid_passwords: i32 = 0;
    for d in data {
        let count:i32 = d.pass.matches(d.letter).count() as i32;
        if d.lower_limit <= count && count <= d.higher_limit {
            valid_passwords += 1;
        }
    }
    valid_passwords
}

fn simple_solution_part_2() -> i32 {
    let data = read_input_data().unwrap();
    let mut valid_passwords: i32 = 0;
    for d in data {
        if (d.pass.chars().nth(d.lower_limit as usize - 1).unwrap() == d.letter)
            ^ (d.pass.chars().nth(d.higher_limit as usize - 1).unwrap() == d.letter) {
            valid_passwords += 1;
        }
    }
    valid_passwords
}


fn main() {
    println!("{}", simple_solution_part_1());
    println!("{}", simple_solution_part_2())
}
