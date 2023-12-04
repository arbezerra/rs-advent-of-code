fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

fn update_value(max: &mut i32, test: i32, value: i32) -> bool{
    *max = value.max(*max);
    return *max > test;
}

fn process(input: &str) -> i32 {
    let mut total = 0;
    let test = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    for line in input.lines() {
        let mut max = Cubes {
            red: 0,
            green: 0,
            blue: 0,
        };
        let split: Vec<&str> = line.split(": ").collect();

        let rounds: Vec<&str> = split[1].split("; ").collect();
        for round in rounds {
            let cubes: Vec<&str> = round.split(", ").collect();
            for cube in cubes {
                let value: Vec<&str> = cube.split(" ").collect();
                let n = value[0].parse::<i32>().unwrap();
                match value[1] {
                    "red" => update_value(&mut max.red, test.red, n),
                    "green" => update_value(&mut max.green, test.green, n),
                    "blue" => update_value(&mut max.blue, test.blue, n),
                    _ => false,
                };
            }
        }

        total += max.red * max.green * max.blue;
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let out = process(input);
        assert_eq!(out, 2286);
    }
}


