use std::fs;
use std::collections::HashSet;

fn clean_nums(nums: &String) -> Vec<u32> {
    nums.split(" ")
        .filter(|&x| x != "")
        .map(|x| x.trim().parse().expect("should be num"))
        .collect()
}

fn vec_to_set<T: std::hash::Hash + std::cmp::Eq + std::clone::Clone>(vec: Vec<T>) -> HashSet<T> {
    HashSet::from_iter(vec.iter().cloned())
}

fn parse_card(card: &str) -> (HashSet<u32>, Vec<u32>) {
    let halves: Vec<&str> = card.split(":").collect();
    let card_vals: Vec<&str> = halves[1].split("|").collect();

    let winners = vec_to_set(clean_nums(&String::from(card_vals[0])));
    let card_nums = clean_nums(&String::from(card_vals[1]));
    
    (winners, card_nums)
}

fn part1(input: &String) {
    let mut total_score = 0;

    for line in input.lines() {
        let (winning_nums, card_nums) = parse_card(line);

        // 2**(len-1)
        let mut wins = 0;
        for num in card_nums.iter() {
            if winning_nums.contains(num) {
                wins += 1;
            }
        }

        if wins > 0 {
            total_score += 2_u32.pow(wins-1);
        }
    }

    println!("result: {total_score}");
}

// ---

fn part2(input: &String) {
    let cards: Vec<&str> = input.lines().collect();
    let mut card_counts = vec![1; cards.len()];


    for (idx, card) in cards.iter().enumerate() {
        let (winning_nums, card_nums) = parse_card(card);

        let mut wins = 0;
        for num in card_nums.iter() {
            if winning_nums.contains(num) {
                wins += 1;
            }
        }

        let num_cards = card_counts[idx];

        // this shouldnt execute when wins==0
        if idx+1 == cards.len() {
            break;
        }
        let end_bound = if idx+1+wins > cards.len() { cards.len() } else { idx+1+wins };
        for card_count in &mut card_counts[(idx+1)..end_bound] {
            // inc card counts
            *card_count += num_cards;
        }

    }


    println!("total scratch cards: {}", card_counts.iter().sum::<u32>());
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("file");
//    let contents = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
//Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
//Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
//Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
//Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
//Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();

    part2(&contents);
}
