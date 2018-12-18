mod aoc;

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
    let mut claims = Vec::new();
    let mut fabric = vec![
        vec![UNCLAIMED;1000]; 1000];

    let mut total_overlap = 0;
    for claim_values in claims_values {
         let inputs: Vec<u32> =
            claim_values.split(|c| "# @,:x".contains(c))
            .filter(|x| !x.is_empty())
            .map(|x|
                 x.parse().expect(x)).collect();
         let mut claim = Claim {
                 id: inputs[0], x: inputs[1], y: inputs[2],
                 width: inputs[3], height: inputs[4], overlap: false };
        for x in 0..claim.width {
            for y in 0..claim.height {
                let x_index = (claim.x + x) as usize;
                let y_index = (claim.y + y) as usize;

                if fabric[x_index][y_index] == UNCLAIMED {
                    fabric[x_index][y_index] = CLAIMED;
                }
                else if fabric[x_index][y_index] == CLAIMED {
                    fabric[x_index][y_index] = OVERLAP;
                    claim.overlap = true;
                    total_overlap += 1;
                }
                else if fabric[x_index][y_index] == OVERLAP {
                    claim.overlap = true;
                }
            }
        }
        if !claim.overlap {
            claims.push(claim);
        }
    }

    for c in claims.iter_mut() {
        for x in c.x..c.end_x() {
            for y in c.y..c.end_y() {
                if fabric[x as usize][y as usize] == 'X' {
                    c.overlap = true;
                    break;
                }
            }
            if c.overlap {
                break
            }
        }
    }
    claims.retain(|c| !c.overlap);

    println!("Day 3 part one result: {}", total_overlap);
    println!("Day 3 part two result: #{}", claims[0].id);
    Ok(())
}
