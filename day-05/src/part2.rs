fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

#[derive(Debug, Clone)]
struct Map {
    start: u64,
    end: u64,
    output: u64,
}

#[derive(Debug, Clone)]
struct Seed {
    start: u64,
    end: u64,
}

fn parse_line(line: &str) -> Map {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let start = parts[1].parse::<u64>().unwrap();
    let size = parts[2].parse::<u64>().unwrap();
    return Map {
        start: start,
        end: start + size,
        output: parts[0].parse::<u64>().unwrap(),
    };
}

fn process(input: &str) -> u64 {
    let mut seeds: Vec<Seed> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();

    let inputs: Vec<&str> = lines[0].split(": ").collect();
    let inputs: Vec<&str> = inputs[1].split(" ").collect();

    for i in (0..inputs.len()).step_by(2) {
        let start = inputs[i].parse::<u64>().unwrap();
        let size = inputs[i + 1].parse::<u64>().unwrap();
        seeds.push(Seed {
            start: start,
            end: start + size,
        });
    }

    let mut blocks: Vec<Vec<Map>> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();
    for i in 3..lines.len() + 1 {
        if i == lines.len() || lines[i].is_empty() {
            maps.sort_by(|a, b| a.start.cmp(&b.start));
            blocks.push(maps.clone());
        } else if lines[i].contains(":") {
            maps = Vec::new();
        } else {
            maps.push(parse_line(lines[i]));
        }
    }

    for maps in blocks.iter() {
        let mut transformed: Vec<Seed> = Vec::new();
        while seeds.len() > 0 {
            let seed = seeds.pop().unwrap();
            let mut not_found = true;
            for map in maps.iter() {
                let start = seed.start.max(map.start);
                let end = seed.end.min(map.end);
                if start < end {
                    transformed.push(Seed {
                        start: start - map.start + map.output,
                        end: end - map.start + map.output,
                    });
                    if start > seed.start {
                        seeds.push(Seed {
                            start: seed.start,
                            end: start,
                        })
                    }
                    if seed.end > end {
                        seeds.push(Seed {
                            start: end,
                            end: seed.end,
                        })
                    }
                    not_found = false;
                    break;
                }
            }
            if not_found {
                transformed.push(seed)
            }
        }
        seeds = transformed;
    }

    seeds.sort_by(|a, b| a.start.cmp(&b.start));

    return seeds[0].start;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4";
        let out = process(input);
        assert_eq!(out, 46);
    }
}
