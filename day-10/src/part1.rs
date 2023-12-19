fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
struct Position {
    i: usize,
    j: usize,
}

fn connects(from: char, to: char, direction: char) -> bool {
    return match from {
        'S' => true,
        '|' => match direction{
            'U' => ['S','7','F','|'].contains(&to),
            'D' => ['S','L','J','|'].contains(&to),
            _ => false,
        },
        '-' => match direction{
            'L' => ['S','L','F','-'].contains(&to),
            'R' => ['S','7','J','-'].contains(&to),
            _ => false,
        },
        'L' => match direction{
            'U' => ['S','7','F','|'].contains(&to),
            'R' => ['S','7','J','-'].contains(&to),
            _ => false,
        },
        'J' => match direction{
            'U' => ['S','7','F','|'].contains(&to),
            'L' => ['S','L','F','-'].contains(&to),
            _ => false,
        },
        '7' => match direction{
            'L' => ['S','L','F','-'].contains(&to),
            'D' => ['S','L','J','|'].contains(&to),
            _ => false,
        },
        'F' => match direction{
            'D' => ['S','L','J','|'].contains(&to),
            'R' => ['S','7','J','-'].contains(&to),
            _ => false,
        },
        _ => false,
    };
}

fn neighbors(matrix: &Vec<Vec<char>>, pos: &Position) -> Vec<Position> {
    let mut neighbors: Vec<Position> = Vec::new();
    // Left
    if pos.j > 0 && connects(matrix[pos.i][pos.j], matrix[pos.i][pos.j - 1], 'L') {
        neighbors.push(Position {
            i: pos.i,
            j: pos.j - 1,
        });
    }
    // Up
    if pos.i > 0 && connects(matrix[pos.i][pos.j], matrix[pos.i - 1][pos.j], 'U') {
        neighbors.push(Position {
            i: pos.i - 1,
            j: pos.j,
        });
    }
    // Right
    if pos.j < matrix[pos.i].len() - 1 && connects(matrix[pos.i][pos.j], matrix[pos.i][pos.j + 1], 'R') {
        neighbors.push(Position {
            i: pos.i,
            j: pos.j + 1,
        });
    }
    // Down
    if pos.i < matrix.len() - 1 && connects(matrix[pos.i][pos.j], matrix[pos.i + 1][pos.j], 'D') {
        neighbors.push(Position {
            i: pos.i + 1,
            j: pos.j,
        });
    }
    return neighbors;
}

fn process(input: &str) -> i64 {
    let mut result = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        matrix.push(line.chars().collect());
    }
    let mut start = Position { i: 0, j: 0 };
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'S' {
                start.i = i;
                start.j = j;
                break;
            }
        }
    }
    let mut at = start.clone();
    let mut last = at.clone();
    loop {
        let neighbors = neighbors(&matrix, &at);
        if neighbors.len() == 0{
            break;
        }
        for n in neighbors.iter() {
            if *n != last {
                last = at.clone();
                at = n.clone();
                result += 1;
                break;
            }
        }
        if at == start {
            break;
        }
    }

    return result / 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let out = process(input);
        assert_eq!(out, 8);
    }
}
