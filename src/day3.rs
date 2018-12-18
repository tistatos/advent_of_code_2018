mod aoc;

use std::collections::HashMap;
use self::aoc::get_string_rows;

const UNCLAIMED: char = '.';
const CLAIMED: char = '#';
const OVERLAP: char = 'X';

struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    overlap: bool
}

impl Claim {
    fn end_x(&self) -> u32 {
        self.x + self.width
    }

    fn end_y(&self) -> u32 {
        self.y + self.height
    }
}

fn main() -> std::io::Result<()> {

    let claims_values = get_string_rows("input_data/day_3");
    let mut fabric = vec![
        vec![UNCLAIMED;1000]; 1000];

    let mut total_overlap = 0;
    let mut non_overlapping_id = 0;
    for claim_values in claims_values {
         let inputs: Vec<u32> =
            claim_values.split(|c|
                c == '#' ||
                c == ' ' ||
                c == '@' ||
                c == ',' ||
                c == ':' ||
                c == 'x' ).filter(|x| !x.is_empty())
            .map(|x|
                 x.parse().expect(x)).collect();
         let claim = Claim {
                 id: inputs[0], x: inputs[1], y: inputs[2],
                 width: inputs[3], height: inputs[4], overlap: false };
        let mut overlapped = false;
        for x in 0..claim.width {
            for y in 0..claim.height {
                let x_index = (claim.x + x) as usize;
                let y_index = (claim.y + y) as usize;

                if fabric[x_index][y_index] == UNCLAIMED {
                    fabric[x_index][y_index] = CLAIMED;
                }
                else if fabric[x_index][y_index] == CLAIMED {
                    fabric[x_index][y_index] = OVERLAP;
                    overlapped = true;
                    total_overlap += 1;

                }
            }
        }
        if !overlapped {
            non_overlapping_id = claim.id;
        }
    }
    println!("Day 3 part one result: {}", total_overlap);
    println!("Day 3 part two result: #{}", non_overlapping_id);
    //for line in fabric.iter() {
        //for dot in line.iter() {
            //print!("{}", dot)
        //}
        //println!("");
    //}

    Ok(())
}
