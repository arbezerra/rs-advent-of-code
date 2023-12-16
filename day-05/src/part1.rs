fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

#[derive(Debug)]
struct Map {
    input: u32,
    output: u32,
    size: u32,
}

fn parse_line(line: &str) -> Map {
    let parts: Vec<&str> = line.split_whitespace().collect();
    return Map {
        input: parts[1].parse::<u32>().unwrap(),
        output: parts[0].parse::<u32>().unwrap(),
        size: parts[2].parse::<u32>().unwrap(),
    };
}

fn process(input: &str) -> u32 {
    let mut result: Vec<u32> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();

    let seeds: Vec<&str> = lines[0].split(": ").collect();
    let seeds: Vec<&str> = seeds[1].split(" ").collect();

    for seed in seeds {
        result.push(seed.parse::<u32>().unwrap());
    }

    let mut maps: Vec<Map> = Vec::new();
    for i in 3..lines.len()+1 {
        if i == lines.len() || lines[i].is_empty(){
            for j in 0..result.len() {
                for map in maps.iter() {
                    if result[j] >= map.input && result[j] < map.input + map.size {
                        result[j] = result[j] - map.input + map.output;
                        break;
                    }
                }
            }
        } else if lines[i].contains(":") {
            maps = Vec::new();
        } else {
            maps.push(parse_line(lines[i]));
        }
    }

    return *result.iter().min().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4";
        let out = process(input);
        assert_eq!(out, 35);
    }
}
