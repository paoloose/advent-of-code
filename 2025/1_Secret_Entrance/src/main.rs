use std::error::Error;

const START_NUM: u32 = 50;

fn rotate_right(current: u32, amount: u32) -> u32 {
    (current + amount) % 100
}

fn part1() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let input = input.trim();

    let mut current = START_NUM;

    let mut zeroes = 0;

    for part in input.split("\n") {
        let (direction_str, amount_str) = part.split_at(1);
        let amount_num = amount_str.parse::<u32>()? % 100;

        let amount_num = {
            if direction_str == "L" {
                100 - amount_num
            } else if direction_str == "R" {
                amount_num
            } else {
                unreachable!("malformed input")
            }
        };

        current = rotate_right(current, amount_num);

        if current == 0 {
            zeroes += 1;
        }
    }

    println!("current = {current}");
    println!("zeroes = {zeroes}");

    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let input = input.trim();

    let mut current = START_NUM;

    let mut zeroes = 0;

    for part in input.split("\n") {
        let (direction_str, amount_str) = part.split_at(1);
        println!("-> {direction_str}{amount_str}");
        let mut amount_num = amount_str.parse::<u32>()?;

        while amount_num >= 100 {
            amount_num -= 100;
            zeroes += 1;
        }

        amount_num = {
            if amount_num == 0 {
                amount_num
            }
            else if direction_str == "L" {
                if (amount_num >= current) && current != 0 {
                    zeroes += 1;
                }
                100 - amount_num
            }
            else if direction_str == "R" {
                if (amount_num + current >= 100) && current != 0 {
                    zeroes += 1;
                }
                amount_num
            }
            else {
                unreachable!("malformed input")
            }
        };
        println!("-> R{amount_num}");

        current = rotate_right(current, amount_num);

        println!("current: {current}");
        println!("zeroes: {zeroes}");
        println!();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // part1();

    part2()?;

    Ok(())
}
