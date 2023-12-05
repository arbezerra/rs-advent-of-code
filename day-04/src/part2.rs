use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

struct Game {
    winner: HashSet<i32>,
    result: i32,
    q: i32,
}

fn process(input: &str) -> i32 {
    let mut total: i32 = 0;
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        let mut g = Game {
            winner: HashSet::new(),
            result: 0,
            q: 1,
        };

        let game: Vec<&str> = line.split(": ").collect();
        let parts: Vec<&str> = game[1].split(" | ").collect();

        for wn in parts[0]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
        {
            g.winner.insert(wn);
        }
        for n in parts[1]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
        {
            if g.winner.contains(&n) {
                g.result += 1;
            }
        }

        games.push(g);
    }
    for i in 0..games.len() {
        let mut j = i + 1;
        total += games[i].q;
        while j < games.len() && games[i].result > 0 {
            games[j].q += games[i].q;
            games[i].result -= 1;
            j += 1;
        }
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let out = process(input);
        assert_eq!(out, 30);
    }
}
