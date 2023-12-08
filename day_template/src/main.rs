use ferris_says::say;
use std::io::{stdout, BufWriter};

/*
*/

fn part_1(lines: Vec<String>) -> bool {
    true
}

fn part_2(lines: Vec<String>) -> bool {
    true
}

fn main() {
    let stdout = stdout();
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    // let result = part_1(lines);
    let result = part_2(lines);
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
        assert!(part_1("".lines().map(|l| l.to_string()).collect()));
    }

    #[test]
    fn test_part_2() {
        assert!(part_2("".lines().map(|l| l.to_string()).collect()));
    }
}
