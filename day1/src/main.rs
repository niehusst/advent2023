use std::fs;

const FPATH: &str = "./input.txt";


fn extract_num(line: &str) -> i32 {
    // these need to be number eventually
    let mut first = String::new();
    let mut last = String::new();

    for c in line.chars() {
        if c.is_digit(10) {
            if first == "" {
                first = c.to_string();
            }
            last = c.to_string();
        }
    }

    (first.to_owned() + &last).parse().expect("should have been a number")
}

fn part1(content: &str) {
    let content_vec = content.lines();

    let mut sum = 0;

    for line in content_vec {
        let calibration = extract_num(line);
        sum += calibration;
    }

    println!("{sum}");
}
// ---
// 

const NUMS: [&str;9] = ["one","two","three","four","five","six","seven","eight","nine"];

fn unwrap<T>(s: Option<T>) -> T {
//    match s {
//        Some(val) => val,
//        None => panic!(),
//    }
    s.unwrap()
}

fn first_num_str(line: &str) -> String {
    let dummy = (usize::MAX, "na");

    let fdigit = match line.match_indices(char::is_numeric).next() {
        Some(fmatch) => fmatch,
        None => dummy,
    };
    let fword = NUMS.map(|word| {
        match line.match_indices(word).next() {
            Some(fmatch) => fmatch,
            None => dummy,
        }
    }).into_iter().reduce(|acc, curr| {
        if acc.0 > curr.0 {
            curr
        } else {
            acc
        }
    }).unwrap();

    // return earlier occurence
    if fdigit > fword {
        match NUMS.into_iter().position(|num_word| num_word == fword.1.to_string()) {
            Some(idx) => (idx + 1).to_string(),
            None => {
                println!("dafucq first? ({}, {}), ({}, {})", fdigit.0, fdigit.1, fword.0, fword.1);
                panic!()
            }
        }
    } else if fdigit < fword {
        fdigit.1.to_string()
    } else {
        println!("dafucq first? {}, {}", fdigit.1, fword.1);
        panic!()
    }
}

fn last_num_str(line: &str) -> String {

    let dummy: (i32, &str) = (-1, "na");

    let ldigit = match line.match_indices(char::is_numeric).last() {
        Some(lmatch) => (i32::try_from(lmatch.0).unwrap(), lmatch.1),
        None => dummy,
    };
    let lword = NUMS.map(|word| {
        match line.match_indices(word).last() {
            Some(lmatch) => (i32::try_from(lmatch.0).unwrap(), lmatch.1),
            None => dummy,
        }
    }).into_iter().reduce(|acc, curr| {
        if acc.0 < curr.0 {
            curr
        } else {
            acc
        }
    }).unwrap();

    // return later occurence
    if ldigit < lword {
        match NUMS.into_iter().position(|num_word| num_word == lword.1.to_string()) {
            Some(idx) => (idx + 1).to_string(),
            None => {
                println!("dafucq first? ({}, {}), ({}, {})", ldigit.0, ldigit.1, lword.0, lword.1);
                panic!()
            }
        }
    } else if ldigit > lword {
        ldigit.1.to_string()
    } else {
        println!("dafucq last? {}, {}", ldigit.1, lword.1);
        panic!()
    }
}

fn extract_num2(line: &str) -> i32 {
    let first = first_num_str(line);
    let last = last_num_str(line);

    (first.to_owned() + &last).parse().expect("should have been a number")
}

fn part2(content: &str) {
    let content_vec = content.lines();

    let mut sum = 0;

    for line in content_vec {
        let calibration = extract_num2(line);
        sum += calibration;
    }

    println!("sum2: {sum}");

}


fn main() {

    let contents = fs::read_to_string(FPATH)
        .expect("Should have been able to read the file");

    let example = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    part2(&contents);
}
