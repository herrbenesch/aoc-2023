use ferris_says::say;
use std::io::{stdout, BufWriter};

/*
*/


fn prepare(lines: Vec<String>) -> Vec<Vec<Vec<u32>>> {
    let mut cards: Vec<Vec<Vec<u32>>> = Vec::new();
    // split each line at : and |
    for line in lines {
        let mut card: Vec<Vec<u32>> = Vec::new();
        let parts: Vec<&str> = line.split(':').collect();
        let numbers: Vec<&str> = parts[1].split('|').collect();
        // get winning numbers, strip whitespace and split at space and remove empty strings
        let winning_numbers: Vec<&str> = numbers[0].trim().split(' ').filter(|s| !s.is_empty()).collect();
        let my_numbers: Vec<&str> = numbers[1].trim().split(' ').filter(|s| !s.is_empty()).collect();
        card.push(winning_numbers.iter().map(|s| s.parse::<u32>().unwrap()).collect());
        card.push(my_numbers.iter().map(|s| s.parse::<u32>().unwrap()).collect());
        cards.push(card)
    }
    cards
}

fn calculate_points(len_common_numbers: u32) -> u32 {
    let mut points = 0;

    for (idx, _) in (0..len_common_numbers).enumerate() {
        if idx == 0 {
            points += 1;
        } else {
            points *= 2;
        }
    }

    points
}


fn part_1(lines: Vec<String>) -> u32 {
    let cards: Vec<Vec<Vec<u32>>> = prepare(lines);
    let mut common: Vec<Vec<u32>> = Vec::new();
    for card in cards {
        // find common numbers
        common.push(card[0].iter().filter(|&n| card[1].contains(n)).cloned().collect());
    }
    let points: Vec<u32> = common.iter().map(|c| calculate_points(c.len() as u32)).collect();
    points.iter().sum::<u32>()
}

// fn part_2(lines: Vec<String>) -> bool {
//     true
// }

fn main() {
    let stdout = stdout();
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    // let result = part_1(lines);
    let result = part_1(lines);
    let message = String::from("The solution is: ") + &result.to_string() + &String::from("\n");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
               .lines().map(|l| l.to_string()).collect()
           ),
        13);
    }

    // #[test]
    // fn test_part_2() {
    //     assert!(part_2("".lines().map(|l| l.to_string()).collect()));
    // }
}
