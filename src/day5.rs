mod aoc;

use self::aoc::get_string;
use std::u32;

const A: u8 = 65;
const Z: u8 = 90;

fn react(input_polymer: &String)  -> usize {
    let mut polymer = input_polymer.clone();
    let mut i = 0;
    while i < (polymer.len() - 1) {
        let a = polymer.chars().nth(i).unwrap();
        let b = polymer.chars().nth(i+1).unwrap();

        if a.to_uppercase().to_string() == b.to_uppercase().to_string() {
            if a.is_uppercase() && b.is_lowercase() ||
                a.is_lowercase() && b.is_uppercase() {
                //reaction occurs
                polymer.remove(i);
                polymer.remove(i);
                if i > 0 {
                    i -= 1;
                }
                continue;
            }
        }
        //no reaction
        i += 1;
    }

    polymer.len()
}

fn trim_unit(unit: char, input_polymer: &String) -> String {
    let uc = unit.to_ascii_uppercase();
    let lc = unit.to_ascii_lowercase();
    let result: String = input_polymer.chars().filter(|&c| c != uc && c != lc).collect();
    result
}

fn main() -> std::io::Result<()> {
    let polymer = get_string("input_data/day_5");
    let mut smallest_polymer = react(&polymer);
    println!("Day 5 part one result: {}", smallest_polymer);

    for c in A..=Z {
        let polymer_len = react(&trim_unit(c as char, &polymer));
        if polymer_len < smallest_polymer {
            smallest_polymer = polymer_len;
        }
    }
    println!("Day 5 part two result: {}", smallest_polymer);

    Ok(())
}
