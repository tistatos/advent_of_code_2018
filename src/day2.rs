mod aoc;

use self::aoc::get_string_rows;

fn main() -> std::io::Result<()> {
    let lines = get_string_rows("input_data/day_2");
    let mut two_times = 0;
    let mut three_times = 0;

    for line in &lines {
        let mut found_two = false;
        let mut found_three = false;
        for i in 0..line.len() {
            let current_letter = line.get(i..i+1).unwrap();
            let mut already_checked = false;
            for j in 0..i {
                let other_letter = line.get(j..j+1).unwrap();
                if current_letter == other_letter {
                    //check if letter in line has already been counted
                    already_checked = true;
                    break;
                }
            }
            if already_checked {
                continue;
            }
            let mut occurence = 1;
            for j in (i+1)..line.len() {
                let other_letter = line.get(j..j+1).unwrap();
                if current_letter == other_letter {
                    occurence += 1;
                }
            }

            if occurence == 2 && !found_two{
                found_two = true;
                two_times += 1;
            }
            else if occurence == 3 && !found_three {
                found_three = true;
                three_times += 1;
            }
        }
    }
    println!("Day 2 part one result: {}", two_times * three_times);



    for i in 0..lines.len() {
        for j in i+1..lines.len() {
            let this_line = &lines[i];
            let that_line = &lines[j];

            let mut sum = 0;
            for (a,b) in this_line.chars().zip(that_line.chars()) {
                if a != b {
                    sum += 1;
                }
            }

            if sum == 1 {
                let mut final_string = String::new();
                for (a,b) in this_line.chars().zip(that_line.chars()) {
                    if a == b {
                        final_string.push(a);
                    }
                }
                println!("{}", this_line);
                println!("{}", final_string);
                println!("Day 2 part two result: {}", final_string);
            }
        }
    }



    Ok(())
}
