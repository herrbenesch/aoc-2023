use ferris_says::say;
use std::collections::{HashMap, HashSet};
use std::io::{stdout, BufWriter};

/*
--- Day 2 - part 1: Cube Conundrum ---
You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many visitors up here; would you like to play a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.

You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration. However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.

Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?

part 2: Cube Conundrum

--- Part Two ---
The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure why the water stopped; however, he can show you how to get to the water source
to check it out for yourself. It's just up ahead!

As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of cubes of each color that could have been in the bag
to make the game possible?

Again consider the example games from earlier:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
Game 4 required at least 14 red, 3 green, and 15 blue cubes.
Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces the sum 2286.

For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?
*/

static INPUT: &str= "Game 1: 2 blue, 3 red; 3 green, 3 blue, 6 red; 4 blue, 6 red; 2 green, 2 blue, 9 red; 2 red, 4 blue
Game 2: 4 red, 1 green; 3 red; 13 green, 5 red, 3 blue; 3 green, 2 red; 3 blue, 5 red, 3 green; 2 red, 3 blue, 12 green
Game 3: 4 red, 1 green, 1 blue; 1 red, 1 blue; 6 red, 1 green; 6 red, 3 blue, 1 green; 4 red
Game 4: 4 blue, 12 red, 4 green; 6 green, 3 blue, 19 red; 3 blue, 2 red, 2 green
Game 5: 1 red, 5 blue, 16 green; 1 red, 6 green, 3 blue; 2 red, 12 blue; 17 blue, 3 green; 7 green, 2 red, 6 blue
Game 6: 3 green, 1 blue, 5 red; 5 green, 5 red; 2 green, 2 blue, 3 red; 5 green, 2 red; 3 green, 6 red, 3 blue; 5 green, 4 red
Game 7: 15 blue, 1 red, 6 green; 4 blue, 7 green, 2 red; 14 blue, 5 green, 2 red
Game 8: 6 blue, 3 green, 10 red; 2 blue, 1 green, 5 red; 6 blue, 3 green, 12 red; 11 red, 1 green, 1 blue; 5 blue, 14 red, 3 green; 3 red
Game 9: 15 red, 3 blue; 1 blue, 16 red; 1 red, 3 blue; 1 blue, 1 green, 9 red
Game 10: 1 red; 1 blue, 7 green; 1 green, 5 blue; 3 blue, 3 green; 1 green
Game 11: 19 blue, 13 green; 19 blue, 2 green; 10 blue, 3 red, 12 green; 11 blue, 1 red, 6 green
Game 12: 7 green, 5 blue; 6 green, 3 red, 6 blue; 2 red, 5 blue, 15 green; 2 red, 1 blue, 1 green; 4 red, 4 green, 2 blue; 3 blue, 6 green
Game 13: 9 red, 2 blue, 2 green; 1 blue, 2 red, 15 green; 9 green, 2 blue, 9 red; 5 blue, 8 green, 5 red; 2 blue, 11 green, 5 red
Game 14: 9 blue, 1 red; 10 blue, 4 green, 3 red; 2 red, 6 blue; 4 green, 2 blue, 1 red; 5 green, 2 red, 11 blue; 12 blue, 2 red, 1 green
Game 15: 9 blue, 7 green, 12 red; 9 red, 17 green, 8 blue; 6 red, 4 blue, 4 green; 5 red, 17 green
Game 16: 5 green, 4 red; 3 blue, 3 red, 14 green; 6 red, 5 blue, 12 green
Game 17: 8 blue, 5 green, 2 red; 6 red, 6 blue; 9 red; 5 blue, 2 green, 8 red; 13 red, 4 blue, 4 green; 9 blue, 3 green, 5 red
Game 18: 8 green, 1 red, 2 blue; 4 green, 4 red, 1 blue; 6 blue, 2 red
Game 19: 3 green, 9 blue; 4 blue, 10 red; 6 red, 3 green, 3 blue; 6 red, 4 green, 9 blue
Game 20: 11 green, 3 blue; 6 green; 3 green, 6 blue; 1 red, 5 green; 6 blue, 7 green
Game 21: 1 green, 1 blue, 12 red; 6 red, 2 blue; 5 green, 4 red, 2 blue; 11 red, 8 green, 1 blue
Game 22: 10 red; 1 red, 13 green, 9 blue; 6 blue, 12 red, 12 green; 10 red, 8 blue, 11 green; 2 green, 1 red, 3 blue; 7 red, 1 blue, 8 green
Game 23: 11 red, 15 blue; 10 blue, 16 red, 1 green; 14 blue, 5 red; 1 green, 9 red, 9 blue; 1 red, 7 blue, 3 green; 6 red, 2 green, 3 blue
Game 24: 6 blue, 11 red; 16 green, 2 red, 1 blue; 8 red, 7 blue; 14 blue, 9 green, 9 red; 13 green, 4 red, 8 blue; 2 red, 7 blue, 1 green
Game 25: 2 green, 12 blue, 1 red; 10 blue, 5 red, 5 green; 2 blue, 9 red, 3 green; 5 blue, 4 red, 2 green
Game 26: 7 blue, 6 red, 1 green; 2 blue, 3 green, 12 red; 2 blue, 6 red, 5 green
Game 27: 2 green, 3 red; 4 green; 2 red, 1 blue, 1 green; 2 red, 1 green, 2 blue
Game 28: 11 blue, 1 red, 5 green; 2 blue, 2 red, 4 green; 10 blue, 4 red, 1 green
Game 29: 6 blue, 17 red, 1 green; 8 blue, 4 red; 14 blue, 1 red, 3 green
Game 30: 2 blue, 4 green; 7 green, 1 blue, 1 red; 1 blue, 8 green
Game 31: 15 blue, 9 green, 2 red; 5 green, 4 blue, 1 red; 1 green, 15 red, 7 blue; 5 red, 2 blue
Game 32: 1 blue, 5 red, 3 green; 3 green, 8 red, 1 blue; 5 green, 1 red; 4 green, 3 blue, 15 red; 2 green, 1 blue; 4 blue, 15 red, 4 green
Game 33: 3 red, 10 blue; 4 red, 9 blue; 1 green, 10 blue
Game 34: 3 blue, 1 green, 9 red; 4 green, 2 red, 9 blue; 7 blue, 3 red; 6 blue, 13 red; 4 green, 13 blue, 9 red
Game 35: 14 red, 1 green; 1 red, 2 green, 4 blue; 3 blue, 10 red, 6 green; 5 blue, 6 red, 7 green; 7 blue, 5 red
Game 36: 2 blue, 8 red, 9 green; 9 green, 3 red, 10 blue; 6 red, 8 blue, 1 green; 6 green, 8 red, 4 blue
Game 37: 10 green, 3 red, 6 blue; 2 blue, 9 red, 5 green; 13 green, 9 red, 10 blue; 2 blue, 4 green, 9 red
Game 38: 4 red, 14 blue, 12 green; 6 red, 12 green, 18 blue; 6 green, 1 blue, 1 red
Game 39: 5 red, 1 blue, 3 green; 1 blue, 3 green, 8 red; 15 red, 1 blue, 5 green; 3 green, 5 red; 1 blue, 14 red; 3 green, 1 blue, 12 red
Game 40: 8 green, 4 blue; 5 blue, 7 red, 8 green; 5 blue, 8 green; 6 green, 3 red, 12 blue; 14 blue, 7 green, 2 red; 1 green, 7 red, 5 blue
Game 41: 7 red, 10 green; 10 red, 6 green; 9 red, 7 green, 1 blue; 3 red, 1 blue
Game 42: 3 green, 2 blue, 13 red; 1 blue, 3 red; 11 green, 16 red; 3 green, 1 blue, 16 red; 5 red, 8 green
Game 43: 12 blue, 9 red; 16 blue, 2 red, 7 green; 4 red, 1 blue, 11 green; 15 blue, 4 red, 9 green
Game 44: 17 green, 5 blue, 2 red; 9 green, 11 blue, 1 red; 20 green, 3 blue, 8 red; 2 red, 13 green, 9 blue; 15 green, 12 blue; 4 blue, 7 green, 9 red
Game 45: 5 green; 5 green, 1 red; 3 green, 2 blue; 1 green, 1 blue, 1 red
Game 46: 10 red, 11 green; 16 green, 8 blue, 12 red; 9 green, 9 blue
Game 47: 20 green, 17 red, 1 blue; 16 red, 2 blue, 11 green; 3 blue, 19 red, 1 green; 3 blue, 17 red, 17 green; 12 green, 2 blue, 7 red
Game 48: 1 red, 4 blue, 6 green; 19 green, 1 red, 1 blue; 16 green, 3 blue, 1 red; 3 blue, 17 green; 4 blue, 12 green
Game 49: 13 green, 2 blue, 1 red; 1 green, 8 red, 2 blue; 11 red, 11 green, 3 blue; 7 red, 8 green, 4 blue
Game 50: 11 blue, 1 red, 2 green; 1 green, 10 blue; 1 blue; 6 blue; 1 green, 2 blue
Game 51: 3 red, 3 green, 1 blue; 3 green, 3 red; 10 green, 4 red; 3 red, 2 green
Game 52: 1 red, 4 blue; 1 green, 11 blue; 1 green, 3 red, 6 blue; 4 red, 1 green, 4 blue; 9 blue, 1 green; 10 blue, 1 green
Game 53: 2 blue, 4 green, 1 red; 8 blue, 4 red, 7 green; 9 red, 7 blue, 6 green; 3 red, 7 green, 1 blue; 2 red, 9 blue, 5 green; 1 green, 7 red, 10 blue
Game 54: 1 red, 1 blue, 5 green; 2 red, 1 green, 2 blue; 3 green, 3 blue, 2 red; 4 red; 12 red, 5 green, 2 blue
Game 55: 2 red, 11 blue; 16 green, 7 red, 16 blue; 4 blue, 11 green, 7 red; 8 green, 18 blue, 8 red
Game 56: 2 blue, 2 green, 1 red; 1 red, 1 green; 1 red; 4 green; 1 blue; 1 blue, 7 green
Game 57: 4 blue, 3 green; 16 green, 2 red, 5 blue; 1 red, 13 green, 2 blue; 3 blue, 12 green, 2 red; 2 red, 5 blue, 4 green; 10 green, 2 blue
Game 58: 3 blue, 8 green; 4 green, 3 blue; 7 green, 5 blue, 5 red; 8 green; 3 red, 6 blue, 9 green; 2 red, 10 green, 4 blue
Game 59: 7 blue, 6 green, 5 red; 7 red, 2 blue; 5 red, 11 green, 14 blue; 8 green, 17 red
Game 60: 3 green, 8 blue, 2 red; 4 green, 7 blue, 6 red; 13 blue, 8 green, 2 red; 10 red, 6 blue, 5 green; 11 green, 3 blue, 4 red; 9 red, 5 green, 9 blue
Game 61: 4 red, 18 blue, 13 green; 9 green, 5 red, 3 blue; 4 green, 3 blue, 4 red; 8 red, 4 green, 7 blue; 8 red, 4 blue, 6 green; 10 green, 5 red, 14 blue
Game 62: 12 red, 14 blue, 9 green; 9 blue, 6 red, 4 green; 2 red, 5 blue; 1 red, 12 blue
Game 63: 11 blue, 13 red, 11 green; 4 blue, 9 green; 8 blue, 9 red; 7 red, 11 green, 7 blue
Game 64: 10 blue, 8 red, 12 green; 10 red, 12 blue, 9 green; 3 green, 17 red; 12 green, 15 blue, 16 red; 6 green, 15 blue, 1 red; 9 red, 6 blue, 10 green
Game 65: 7 red, 7 blue; 3 blue, 1 red, 1 green; 3 red, 8 blue
Game 66: 1 blue, 3 red; 10 green, 5 blue; 4 green; 3 red, 11 green; 3 blue, 15 green, 3 red
Game 67: 1 red; 2 blue, 2 green, 1 red; 6 green, 1 blue
Game 68: 7 red, 4 blue; 4 blue, 6 red, 7 green; 2 green, 19 red, 11 blue; 11 green, 9 red
Game 69: 4 blue, 3 green, 1 red; 7 blue, 1 red, 3 green; 5 blue, 1 green; 2 blue, 10 green, 2 red; 2 red, 6 green, 5 blue; 1 red, 4 green, 2 blue
Game 70: 9 blue, 7 red, 6 green; 19 blue, 4 red, 5 green; 6 blue, 7 red, 4 green; 3 blue, 4 red, 2 green
Game 71: 6 green, 12 blue, 4 red; 11 red, 10 green, 11 blue; 3 red, 14 blue, 13 green; 4 blue, 3 green
Game 72: 2 green, 1 blue, 9 red; 10 red, 3 green, 1 blue; 11 red, 2 green; 2 green, 1 blue, 5 red; 1 red, 1 blue, 3 green; 13 red, 4 blue, 1 green
Game 73: 11 green, 6 blue; 7 green, 6 blue, 7 red; 12 green, 8 blue, 11 red; 4 red, 2 blue, 9 green; 4 green, 7 blue, 2 red
Game 74: 3 blue, 7 red; 3 blue, 5 green, 2 red; 5 red, 1 green, 3 blue; 8 green, 2 blue, 11 red; 3 blue, 8 green, 10 red
Game 75: 2 green; 5 blue; 1 blue, 1 red; 1 red, 9 blue, 2 green; 2 blue, 2 green
Game 76: 12 blue, 13 green; 5 red, 11 blue, 9 green; 12 green, 6 red
Game 77: 1 blue, 15 green, 12 red; 15 green, 5 blue; 14 green, 3 blue, 8 red
Game 78: 11 green, 8 blue, 1 red; 9 green, 8 blue, 1 red; 13 green, 5 red, 6 blue; 5 red, 7 green, 20 blue; 10 blue, 5 red
Game 79: 3 blue; 6 blue, 5 red; 4 red, 1 green, 4 blue; 7 blue, 6 red; 7 red, 1 blue; 1 red, 1 blue, 1 green
Game 80: 11 green, 3 red, 8 blue; 2 red, 15 green, 2 blue; 5 green, 8 blue, 2 red; 8 blue, 14 green; 2 blue, 13 green
Game 81: 9 red, 4 green; 7 green, 4 red; 2 red, 4 blue, 6 green; 6 red, 4 blue, 9 green; 1 green, 3 red; 6 green, 1 blue, 8 red
Game 82: 5 blue, 3 red, 3 green; 5 red; 2 red, 3 green, 8 blue
Game 83: 10 green, 1 red, 1 blue; 3 red, 1 green, 1 blue; 4 red, 10 green
Game 84: 16 red, 2 green, 6 blue; 6 red, 3 green, 8 blue; 3 green, 10 red, 5 blue; 4 blue, 3 green; 15 red
Game 85: 3 green, 2 red; 5 green, 4 blue; 5 green, 8 red, 3 blue
Game 86: 7 green, 16 blue, 7 red; 1 green, 12 red, 2 blue; 15 green, 16 blue, 7 red
Game 87: 1 red, 6 green, 5 blue; 2 green, 1 blue; 2 green, 1 red, 1 blue; 5 green, 4 blue
Game 88: 3 green, 3 red, 4 blue; 1 red, 1 green; 6 blue, 9 red, 1 green; 1 green, 11 red, 3 blue; 7 red, 6 blue
Game 89: 2 blue, 3 red, 4 green; 5 red, 7 blue, 14 green; 8 blue, 5 red, 16 green; 2 blue, 5 red, 7 green; 5 green, 9 blue, 1 red
Game 90: 1 blue, 3 red, 7 green; 11 green, 4 red, 1 blue; 1 red, 1 blue, 6 green; 2 blue, 2 green; 8 green, 2 blue; 3 red, 2 blue, 4 green
Game 91: 6 blue, 4 red, 1 green; 8 red, 3 blue, 3 green; 1 green, 2 blue, 5 red; 1 blue, 3 green
Game 92: 8 green, 1 red, 5 blue; 2 green, 7 blue; 11 blue, 5 green, 8 red; 7 blue, 3 red, 4 green
Game 93: 3 green, 1 red, 9 blue; 13 red, 5 blue, 8 green; 5 green, 2 red, 7 blue
Game 94: 4 green, 10 blue, 8 red; 4 red, 10 blue, 2 green; 2 green, 10 blue, 5 red; 5 green, 2 red, 10 blue
Game 95: 5 green, 1 blue; 3 blue, 11 green, 8 red; 8 blue, 2 red, 12 green; 4 green, 4 blue, 4 red
Game 96: 1 blue, 13 green; 8 blue, 3 red, 4 green; 1 red, 3 blue, 10 green
Game 97: 18 green, 4 red; 1 blue, 2 red, 9 green; 6 red, 3 blue, 10 green; 3 blue, 15 green, 4 red
Game 98: 2 blue, 3 green, 6 red; 1 green, 1 blue, 8 red; 8 red, 3 green, 1 blue; 2 blue; 8 red, 2 green, 2 blue
Game 99: 1 green, 2 red, 1 blue; 8 green, 4 blue, 1 red; 7 blue, 1 red, 11 green; 9 green, 3 blue; 1 red, 2 blue; 1 red, 6 blue
Game 100: 7 blue, 9 green, 2 red; 5 red, 9 green; 1 blue, 8 red, 13 green";


fn prepare_games(input: &str) -> HashMap<String, Vec<Vec<String>>> {
    input
        .lines()
        .map(|element| {
            let mut parts = element.splitn(2, ':').map(str::trim);
            let game_id = parts
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .to_string();
            let game_elements = parts
                .next()
                .unwrap()
                .split(';')
                .map(str::trim)
                .map(|round| round.split(',').map(str::trim).map(String::from).collect())
                .collect();
            (game_id, game_elements)
        })
        .collect()
}

fn is_game_possible(game_elements: &Vec<Vec<String>>, bag: &HashMap<&str, i32>) -> bool {
    for round_move in game_elements.iter().flatten() {
        let parts: Vec<&str> = round_move.split_whitespace().collect();
        if let Ok(value) = parts[0].parse::<i32>() {
            let color = parts[1];
            if let Some(color_bag) = bag.get(color) {
                if value > *color_bag {
                    return false;
                }
            }
        }
    }
    true
}

fn calculate_product(game_elements: &Vec<Vec<String>>) -> i32 {
    let mut max_values: HashMap<&str, i32> = [("red", 0), ("green", 0), ("blue", 0)]
        .iter()
        .cloned()
        .collect();

    for round_move in game_elements.iter().flatten() {
        let parts: Vec<&str> = round_move.split_whitespace().collect();
        if let Ok(value) = parts[0].parse::<i32>() {
            let color = parts[1];
            if value > *max_values.get(color).unwrap() {
                max_values.insert(color, value);
            }
        }
    }

    max_values.values().product()
}

fn part_1(input: &str) -> u32 {
    let games = prepare_games(input);
    let mut keys: HashSet<String> = games.keys().cloned().collect();

    let bag: HashMap<&str, i32> = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect();

    for (game_id, game_elements) in games.iter() {
        if !is_game_possible(game_elements, &bag) {
            keys.remove(game_id);
            println!("Game {} is not possible!", game_id);
        }
    }

    keys.iter().map(|x| x.parse::<u32>().unwrap()).sum()
}

fn part_2(input: &str) -> u32 {
    let games = prepare_games(input);
    let products: Vec<i32> = games.values().map(|game_elements| calculate_product(game_elements)).collect();
    products.iter().map(|&x| x as u32).sum()
}


fn main() {
    let stdout = stdout();

    let result = part_1(INPUT);
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
        let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = super::part_1(input);
        assert_eq!(result, 8)
    }

    #[test]
    fn test_part_2() {
        let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = super::part_2(input);
        assert_eq!(result, 2286)
    }
}
