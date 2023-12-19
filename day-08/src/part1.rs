use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> i32 {
    let mut result = 0;
    let mut map: HashMap<&str, Vec<_>> = HashMap::new();
    let lines: Vec<_> = input.lines().collect();
    let moves: Vec<char> = lines[0].chars().collect();
    for i in 2..lines.len() {
        let parts: Vec<&str> = lines[i].split(" = ").collect();
        let value = &parts[1][1..parts[1].len() - 1];
        map.insert(parts[0], value.split(", ").collect());
    }
    let mut at = "AAA";
    let mut move_at = 0;
    while at != "ZZZ" {
        at = match moves[move_at] {
            'L' => map.get(at).unwrap()[0],
            'R' => map.get(at).unwrap()[1],
            _ => "",
        };
        move_at = (move_at + 1) % moves.len();
        result += 1;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let out = process(input);
        assert_eq!(out, 2);
    }
}
