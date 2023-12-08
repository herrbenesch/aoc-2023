use ferris_says::say;
use std::io::{stdout, BufWriter};

/*
*/

fn part_1(input: &str) -> u32 {
    return true:
}

fn part_2(input: &str) -> u32 {
    return true;
}

fn main() {
    let stdout = stdout();

    // let result = part_1(INPUT);
    // let result = part_2(INPUT);
    let message = String::from("The solution is: ") + &result.to_string() + &String::from("\n");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(""), true);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(""), true);
    }
}
