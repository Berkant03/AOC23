use std::fs::File;
use std::io::prelude::*;

fn task01() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut sum: u32 = 0;
    contents.lines().for_each(|x| {
        for c in x.chars() {
            if c.is_digit(10) {
                sum += c.to_digit(10).unwrap() * 10u32;
                break;
            }
        }

        for c in x.chars().rev() {
            if c.is_digit(10) {
                sum += c.to_digit(10).unwrap();
                break;
            }
        }
    });

    println!("{}", sum);

    Ok(())
}

fn task02() -> std::io::Result<()> {
    let numbers = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut sum: u32 = 0;

    contents.lines().for_each(|x| {
        let l_digit = x.find(|c: char| c.is_digit(10));
        let r_digit = x.rfind(|c: char| c.is_digit(10));

        let (left, _) = x.split_at(l_digit.unwrap_or(x.len()-1));
        let (_, right) = x.split_at(r_digit.unwrap_or(0));

        let mut left_index: Option<usize> = None;
        let mut left_num = 0;
        let mut right_index: Option<usize> = None;
        let mut right_num = 0;

        for (i, number) in numbers.iter().enumerate() {
            let pos = left.find(number);
            if left_index.is_none() {
                left_index = pos;
                left_num = i;
            } 
            if pos.is_some() && left_index.unwrap() > pos.unwrap() {
                left_index = pos;
                left_num = i;
            }

            let pos = right.rfind(number);

            if right_index.is_none() {
                right_index = pos;
                right_num = i;
            }

            if pos.is_some() && right_index.unwrap() < pos.unwrap() {
                right_index = pos;
                right_num = i;
            }
        }

        if left_index.is_some() {
            sum += (left_num as u32 + 1) * 10u32;
        } else {
            sum += x.chars().nth(l_digit.unwrap()).unwrap().to_digit(10).unwrap() * 10u32;
        }

        if right_index.is_some() {
            sum += right_num as u32 + 1
        } else {
            sum += x.chars().nth(r_digit.unwrap()).unwrap().to_digit(10).unwrap();
        }
    });

    println!("{}", sum);
    Ok(())
}

fn main() -> std::io::Result<()> {
    task01().unwrap();
    task02().unwrap();

    Ok(())
}