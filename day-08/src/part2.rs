use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn get_lcm(results: Vec<u64>) -> u64 {
    let mut lcm = results[0];
    for i in 1..results.len() {
        lcm = lcm * results[i] / gcd(lcm, results[i]);
    }
    return lcm;
}

fn process(input: &str) -> u64 {
    let mut results = Vec::new();
    let mut map: HashMap<&str, Vec<_>> = HashMap::new();
    let lines: Vec<_> = input.lines().collect();
    let moves: Vec<char> = lines[0].chars().collect();
    let mut at = Vec::new();
    for i in 2..lines.len() {
        let parts: Vec<&str> = lines[i].split(" = ").collect();
        let value = &parts[1][1..parts[1].len() - 1];
        if parts[0].chars().nth(2).unwrap() == 'A' {
            at.push(parts[0]);
        }
        map.insert(parts[0], value.split(", ").collect());
    }
    let mut move_at = 0;
    for i in 0..at.len() {
        let mut result = 0;
        while at[i].chars().nth(2).unwrap() != 'Z' {
            at[i] = match moves[move_at] {
                'L' => map.get(at[i]).unwrap()[0],
                'R' => map.get(at[i]).unwrap()[1],
                _ => "",
            };
            move_at = (move_at + 1) % moves.len();
            result += 1;
        }
        results.push(result);
    }
    dbg!(&results);
    let lcm = get_lcm(results);
    return lcm;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let out = process(input);
        assert_eq!(out, 6);
    }
}
