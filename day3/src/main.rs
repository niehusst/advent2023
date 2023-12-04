use std::fs;
use std::collections::HashSet;

fn is_symbol(c: &char) -> bool {
    *c != '.' && c.is_ascii_punctuation()
}


fn check_neighbors(mat: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    let positions = [
    // up
        if r == 0 { None } else { Some((r-1, c)) },
    // up right
        if r == 0 { None } else { Some((r-1, c+1)) },
    // right
        Some((r, c+1)),
    // down right
        Some((r+1, c+1)),
    // down
        Some((r+1, c)),
    // down left
        if c == 0 { None } else { Some((r+1, c-1)) },
    // left
        if c == 0 { None } else { Some((r, c-1)) },
    // up left
        if c == 0 || r == 0 { None } else { Some((r-1, c-1)) },
    ];

    positions.iter().fold(false, |acc, maybe_coord| {
        let mut any_sym: bool = false;

        if let Some((r, c)) = maybe_coord {
            if let Some(line) = mat.get(*r) {
                if let Some(ch) = line.get(*c) {
                    any_sym = is_symbol(ch)
                }
            }
        }

        acc || any_sym
    })
}

fn part1(input: &String) {
    let mut parts_sum = 0;
    let input_2d: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for row in 0..input_2d.len() {
        let mut num_start = 0;
        let mut num_end = 0;
        let mut tracking = false;
        let mut is_part_number = false;

        for col in 0..input_2d[0].len() {

            if input_2d[row][col].is_digit(10) {
                is_part_number = is_part_number || check_neighbors(&input_2d, row, col);

                if !tracking {
                    num_start = col;
                    tracking = true;
                }

                num_end = col+1;
            } else {
                if tracking {
                    tracking = false;

                    if is_part_number {
                        // extract number
                        let num_str: &String = &input_2d[row][num_start..num_end].iter().cloned().collect::<String>();
                        parts_sum += &num_str[..].parse().expect("should be num");
                    }
                }
                is_part_number = false;
            }

        }

        if tracking {
            tracking = false;

            if is_part_number {
                // extract number str from char array
                let num_str: &String = &input_2d[row][num_start..num_end].iter().cloned().collect::<String>();
                parts_sum += &num_str[..].parse().expect("should be num");
            }
        }
        is_part_number = false;
    }


    println!("parts sum is {}", parts_sum);
}

// ----

fn digits_left(seen: &mut HashSet<String>, line: &Vec<char>, r: usize, c: usize) -> String {
    let mut built_chars: Vec<char> = Vec::new();
    let mut c = c;

    loop {
        let coord_key = format!("{r},{c}");

        if seen.contains(&coord_key) {
            break;
        }
        seen.insert(coord_key);

        if let Some(ch) = line.get(c) {
            if ch.is_digit(10) {
                built_chars.push(*ch);
            } else {
                break;
            }
        } else {
            break;
        }

        // just to prevent usize underflow
        if c == 0 {
            break;
        }

        c -= 1;
    }

    // gotta reverse number since we built up vec backwards
    built_chars.iter().rev().cloned().collect::<String>()
}

fn digits_right(seen: &mut HashSet<String>, line: &Vec<char>, r: usize, c: usize) -> String {
    let mut built_chars: Vec<char> = Vec::new();
    let mut c = c;

    loop {
        let coord_key = format!("{r},{c}");

        if seen.contains(&coord_key) {
            break;
        }
        seen.insert(coord_key);

        if let Some(ch) = line.get(c) {
            if ch.is_digit(10) {
                built_chars.push(*ch);
            } else {
                break;
            }
        } else {
            break;
        }

        c += 1;
    }

    built_chars.iter().cloned().collect::<String>()
}

/// given gear coords r,c in mat, find all numbers surounding the gear
fn surround_nums(mat: &Vec<Vec<char>>, r: usize, c: usize) -> Vec<u32> {

    let positions = [
    // up left
        if c == 0 || r == 0 { None } else { Some((r-1, c-1)) },
    // up
        if r == 0 { None } else { Some((r-1, c)) },
    // up right
        if r == 0 { None } else { Some((r-1, c+1)) },
    // right
        Some((r, c+1)),
    // down left
        if c == 0 { None } else { Some((r+1, c-1)) },
    // down
        Some((r+1, c)),
    // down right
        Some((r+1, c+1)),
    // left
        if c == 0 { None } else { Some((r, c-1)) },
    ];

    let mut seen = HashSet::new();
    let mut nums: Vec<u32> = Vec::new();

    for pos in positions.iter() {
        // i love if statments
        if let Some((r, c)) = pos {
            if let Some(line) = mat.get(*r) {
                if let Some(ch) = line.get(*c) {
                    let coord_key = format!("{r},{c}");

                    if ch.is_digit(10) && !seen.contains(&coord_key) {

                        let dleft = digits_left(&mut seen, &line, *r, *c);
                        let dright = digits_right(&mut seen, &line, *r, *c+1);

                        // after bcus digits_left should take care of adding r,c to seen
                        seen.insert(coord_key);

                        // construct full num str and convert to i32, and add to nums
                        let num = (dleft + &dright).parse().expect("should be number");
                        nums.push(num);
                    }
                }
            }
        }
    }

    nums
}

fn part2(input: &String) {
    // find all gears, check if next to only 2 numbers (account for adj to multi digits in
    // same num), parse out those numbers

    let input_2d: Vec<&str> = input.lines().collect();

    // vec of coords of possible gears
    let mut gear_coords: Vec<(usize, usize)> = Vec::new();

    for (row, line) in input_2d.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '*' {
                gear_coords.push((row, col));
            }
        }
    }

    let input_2d: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut gear_ratios: Vec<u32> = Vec::new();
    for (r, c) in gear_coords {
        // check how many nums around this *
        let nums = surround_nums(&input_2d, r, c);
        //println!("coord ({},{}) gear has adj nums: {:?}", r, c, nums);

        if nums.len() == 2 {
            // we got a gear baybeeee
            gear_ratios.push(nums[0] * nums[1]);
        }
    }

    let gear_sum: u32 = gear_ratios.iter().sum::<u32>();
    println!("gear ratio sum: {}", gear_sum);
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("file should exist");
//    let contents = "467..114..
//...*......
//..35..633.
//......#...
//617*......
//.....+.58.
//..592.....
//......755.
//...$.*....
//.664.598..".to_string();

    part2(&contents);
}


