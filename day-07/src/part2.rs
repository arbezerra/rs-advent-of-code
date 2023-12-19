use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn get_map() -> HashMap<char, i32> {
    let map: HashMap<char, i32> = HashMap::from([
        ('J', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);
    return map;
}

#[derive(Debug)]
struct Game {
    cards: Vec<char>,
    t: i32,
    bid: i32,
}

fn get_type(cards: &Vec<char>) -> i32 {
    let mut map = HashMap::new();
    let mut j = 0;
    for card in cards {
        if *card == 'J' {
            j += 1;
        } else {
            map.entry(card).or_insert(vec![]).push(card);
        }
    }

    let mut values: Vec<usize> = map.iter().map(|e| e.1.len()).collect();
    values.sort();
    values.reverse();

    // Five of a kind
    if j == 5 || values[0] == 5 - j {
        return 7;
    }
    if values.len() > 1 {
        // Four of a kind
        if values[0] == 4 - j {
            return 6;
        }
        // Full house
        if (values[0] == 3 - j && values[1] == 2) || (values[0] == 3 && values[1] == 2 - j) {
            return 5;
        }
        // Three of a kind
        if values[0] == 3 - j {
            return 4;
        }
        // Two pairs
        if values[0] == 2 && values[1] == 2 - j {
            return 3;
        }
        // One pair
        if values[0] == 2 - j {
            return 2;
        }
    }
    return 1;
}

fn process(input: &str) -> i32 {
    let mut total: i32 = 0;
    let map = get_map();
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let bid = parts[1].parse::<i32>().unwrap();
        let cards: Vec<char> = parts[0].chars().collect();
        let t = get_type(&cards);
        let game = Game { cards, t, bid };
        games.push(game);
    }
    games.sort_unstable_by_key(|item| {
        (
            item.t,
            map.get(&item.cards[0]).unwrap(),
            map.get(&item.cards[1]).unwrap(),
            map.get(&item.cards[2]).unwrap(),
            map.get(&item.cards[3]).unwrap(),
            map.get(&item.cards[4]).unwrap(),
        )
    });
    for i in 0..games.len() {
        total += games[i].bid * (i as i32 + 1)
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        let out = process(input);
        assert_eq!(out, 5905);
    }
}
