use std::collections::HashMap;

/**
 * max_cubes takes numbers as (red, green, blue)
 */
fn part1(file_content: &str, max_cubes: (u32, u32, u32)) -> u32 {
    return file_content
        .lines()
        .enumerate()
        .map(|(i, line)| {
            for game in line.split(":").last().unwrap().split(";") {
                for pull in game.split(",").map(|s| s.trim().to_lowercase()) {
                    let (amount, color) = pull.split_at(pull.find(" ").unwrap());
                    let amount: u32 = amount.parse().unwrap();
                    let color = color.trim();

                    let too_many = match color {
                        "red" => amount > max_cubes.0,
                        "green" => amount > max_cubes.1,
                        "blue" => amount > max_cubes.2,
                        _ => panic!("Number expected"),
                    };

                    if too_many {
                        return 0;
                    }
                }
            }

            return (i + 1) as u32;
        })
        .sum();
}

/**
 * max_cubes takes numbers as (red, green, blue)
 */
fn part2(file_content: &str) -> u32 {
    return file_content
        .lines()
        .map(|line| {
            let colormap: HashMap<&str, usize> =
                HashMap::from([("red", 0), ("green", 1), ("blue", 2)]);

            let mut sum: u32 = 1;
            let mut minimum_amount: [u32; 3] = [0, 0, 0];

            for game in line.split(":").last().unwrap().split(";") {
                for pull in game.split(",").map(|s| s.trim().to_lowercase()) {
                    let (amount, color) = pull.split_at(pull.find(" ").unwrap());

                    let amount: u32 = amount.parse().unwrap();
                    let index = colormap.get(color.trim()).unwrap().clone();

                    minimum_amount[index] = std::cmp::max(minimum_amount[index], amount)
                }
            }

            minimum_amount.iter().for_each(|v| {
                sum *= v;
            });

            return sum;
        })
        .sum();
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();

    let num = part1(&contents, (12, 13, 14));
    println!("{}", num);

    let num = part2(&contents);
    println!("{}", num);
}
