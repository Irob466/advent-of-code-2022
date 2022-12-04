use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // PART 1 OF 2 -- The Tournament --
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut score = 0u32;

    for line in reader.lines() {
        if let Ok(l) = line {
            let (str_p2, str_p1) = l.split_at(1);
            let (p2, p1) = (convert_to_number(str_p2), convert_to_number(str_p1));
            let result = p1 - p2;
            match result {
                2 | -1 => score += p1 as u32,
                1 | -2 => score += (p1 + 6) as u32,
                0 => score += (p1 + 3) as u32,
                _ => (),
            }
        }
    }
    println!("Part 1 result: {score}");

    // PART 2 OF 2
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut score = 0u32;

    for line in reader.lines() {
        if let Ok(l) = line {
            let (str_p2, str_res) = l.split_at(1);
            let (p2, res) = (convert_to_number(str_p2), convert_to_result(str_res));
            let p1 = p2 + res;
            match p1 {
                2 | 4 | 9 => score += (1 + res) as u32,
                3 | 5 | 7 => score += (2 + res) as u32,
                1 | 6 | 8 => score += (3 + res) as u32,
                _ => continue,
            }
        }
    }
    println!("Part 2 result: {score}");
    Ok(())
}

fn convert_to_number(play: &str) -> i8 {
    match play.trim() {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0,
    }
}

fn convert_to_result(play: &str) -> i8 {
    match play.trim() {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0,
    }
}
