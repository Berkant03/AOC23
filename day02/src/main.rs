use std::collections::HashMap;

/**
 * max_cubes takes numbers as (red, green, blue)
 */
fn part1(file_content: &str, max_cubes: (u32, u32, u32)) -> u32 {
    file_content
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            for game in line.split(":").last().unwrap().split(";") {
                for pull in game.split(",").map(|s| s.trim().to_lowercase()) {
                    let (amount, color) = pull.split_at(pull.find(" ").unwrap());
                    let amount: u32 = amount.parse().unwrap();
                    let color = color.trim();

                    let color_amt = match color {
                        "red" => max_cubes.0,
                        "green" => max_cubes.1,
                        "blue" => max_cubes.2,
                        _ => panic!("Number expected"),
                    };

                    if amount > color_amt {
                        return None;
                    }
                }
            }

            Some((i + 1) as u32)
        })
        .sum()
}

fn part2(file_content: &str) -> u32 {
    let colormap: HashMap<&str, usize> = HashMap::from([("red", 0), ("green", 1), ("blue", 2)]);
    file_content
        .lines()
        .map(|line| {
            let mut minimum_amount: [u32; 3] = [0, 0, 0];

            for game in line.split(":").last().unwrap().split(";") {
                for pull in game.split(",").map(|s| s.trim().to_lowercase()) {
                    let (amount, color) = pull.split_at(pull.find(" ").unwrap());

                    let amount: u32 = amount.parse().unwrap();
                    let index = *colormap.get(color.trim()).unwrap();

                    let amt_ref = &mut minimum_amount[index];
                    *amt_ref = std::cmp::max(*amt_ref, amount)
                }
            }

            minimum_amount.iter().fold(1, |x, v| x * v)
        })
        .sum()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();

    let num = part1(&contents, (12, 13, 14));
    println!("{}", num);

    let num = part2(&contents);
    println!("{}", num);
}
