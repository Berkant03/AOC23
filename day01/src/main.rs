use std::fs::File;
use std::io::prelude::*;

fn task01() -> std::io::Result<()> {
    let contents = std::fs::read_to_string("input.txt")?;
    let sum = contents.lines().fold(0, |sum, x| {
        sum + 10 * x.chars().find_map(|c| c.to_digit(10)).unwrap()
            + x.chars().rev().find_map(|c| c.to_digit(10)).unwrap()
    });

    println!("{}", sum);

    Ok(())
}

fn task02() -> std::io::Result<()> {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let contents = std::fs::read_to_string("input.txt")?;


    let sum = contents.lines().fold(0, |sum, x| {
        let l_digit = x.find(|c: char| c.is_digit(10)).unwrap();
        let r_digit = x.rfind(|c: char| c.is_digit(10)).unwrap();

        let (left, _) = x.split_at(l_digit);
        let (_, right) = x.split_at(r_digit);

        // index, value
        let mut left_n = (
            l_digit,
            x.chars().nth(l_digit).unwrap().to_digit(10).unwrap(),
        );
        let mut right_n = (-1, x.chars().nth(r_digit).unwrap().to_digit(10).unwrap());

        for (i, number) in numbers.iter().enumerate() {
            let i = (i + 1) as u32;

            if let Some(pos) = left.find(number) {
                // potential improvement: #![feature(let_chains)]
                // https://github.com/rust-lang/rust/issues/53667
                if left_n.0 > pos {
                    left_n = (pos, i);
                }
            };

            if let Some(pos) = right.rfind(number) {
                let pos = pos as isize;
                if right_n.0 < pos {
                    right_n = (pos, i);
                }
            };
        }

        sum + 10 * left_n.1 + right_n.1
    });

    println!("{}", sum);
    Ok(())
}

fn main() -> std::io::Result<()> {
    task01()?;
    task02()?;

    Ok(())
}