use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn get_map() -> HashMap<char, i32> {
    let map: HashMap<char, i32> = HashMap::from([
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('J', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);
    return map;
}

#[derive(Debug)]
struct Game {
    cards: Vec<char>,
    original: Vec<char>,
    t: i32,
    bid: i32,
}

fn get_type(cards: &Vec<char>) -> i32 {
    if cards[0] == cards[1] && cards[1] == cards[2] && cards[2] == cards[3] && cards[3] == cards[4]
    {
        return 7;
    }
    if (cards[0] == cards[1] && cards[1] == cards[2] && cards[2] == cards[3])
        || (cards[1] == cards[2] && cards[2] == cards[3] && cards[3] == cards[4])
    {
        return 6;
    }
    if (cards[0] == cards[1] && cards[0] == cards[2] && cards[3] == cards[4])
        || (cards[0] == cards[1] && cards[2] == cards[3] && cards[3] == cards[4])
    {
        return 5;
    }
    if (cards[0] == cards[1] && cards[0] == cards[2])
        || (cards[1] == cards[2] && cards[1] == cards[3])
        || (cards[2] == cards[3] && cards[2] == cards[4])
    {
        return 4;
    }
    if (cards[0] == cards[1] && cards[2] == cards[3])
        || (cards[0] == cards[1] && cards[3] == cards[4])
        || (cards[1] == cards[2] && cards[3] == cards[4])
    {
        return 3;
    }
    if cards[0] == cards[1] || cards[1] == cards[2] || cards[2] == cards[3] || cards[3] == cards[4]
    {
        return 2;
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
        let mut cards: Vec<char> = parts[0].chars().collect();
        let original: Vec<char> = cards.clone();
        cards.sort_by(|a, b| map.get(b).unwrap().cmp(map.get(a).unwrap()));
        let t = get_type(&cards);
        let game = Game {
            cards: cards,
            original: original,
            t: t,
            bid: bid,
        };
        games.push(game);
    }
    games.sort_unstable_by_key(|item| {
        (
            item.t,
            map.get(&item.original[0]).unwrap(),
            map.get(&item.original[1]).unwrap(),
            map.get(&item.original[2]).unwrap(),
            map.get(&item.original[3]).unwrap(),
            map.get(&item.original[4]).unwrap(),
        )
    });
    for i in 0..games.len() {
        dbg!(&games[i]);
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
        assert_eq!(out, 6440);
    }
}
