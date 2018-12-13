mod aoc;

use std::collections::HashMap;
use self::aoc::get_string_rows;

fn main() -> std::io::Result<()> {
    let lines = get_string_rows("input_data/day_1");


    let numbers: Vec<i64> = lines.iter().map(|l| l.parse::<i64>().unwrap()).collect();
    let frequency: i64 = numbers.iter().sum();
    println!("Day 1 part one result: {}", frequency);

    let mut frequencies = HashMap::new();
    let mut current_frequency = 0;

    let mut i = 0;
    loop {
        current_frequency += numbers[i];
        if frequencies.contains_key(&current_frequency) {
            break;
        }
        frequencies.insert(current_frequency, true);
        i = (i + 1) % numbers.len() ;
    }
    println!("Day 1 part two result: {}", current_frequency);

    Ok(())
}
