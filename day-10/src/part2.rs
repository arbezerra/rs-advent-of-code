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
        'S' => match direction{
            'L' => ['-','F','L'].contains(&to),
            'U' => ['|','F','7'].contains(&to),
            'R' => ['-','J','7'].contains(&to),
            'D' => ['|','L','J'].contains(&to),
            _ => false,
        },
        '|' => match direction {
            'U' => ['7', 'F', '|'].contains(&to),
            'D' => ['L', 'J', '|'].contains(&to),
            _ => false,
        },
        '-' => match direction {
            'L' => ['L', 'F', '-'].contains(&to),
            'R' => ['7', 'J', '-'].contains(&to),
            _ => false,
        },
        'L' => match direction {
            'U' => ['7', 'F', '|'].contains(&to),
            'R' => ['7', 'J', '-'].contains(&to),
            _ => false,
        },
        'J' => match direction {
            'U' => ['7', 'F', '|'].contains(&to),
            'L' => ['L', 'F', '-'].contains(&to),
            _ => false,
        },
        '7' => match direction {
            'L' => ['L', 'F', '-'].contains(&to),
            'D' => ['L', 'J', '|'].contains(&to),
            _ => false,
        },
        'F' => match direction {
            'D' => ['L', 'J', '|'].contains(&to),
            'R' => ['7', 'J', '-'].contains(&to),
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
    if pos.j < matrix[pos.i].len() - 1
        && connects(matrix[pos.i][pos.j], matrix[pos.i][pos.j + 1], 'R')
    {
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

fn find_start_char(matrix: &Vec<Vec<char>>, pos: &Position) -> char {
    let left = pos.j > 0 && connects(matrix[pos.i][pos.j], matrix[pos.i][pos.j - 1], 'L');
    let up = pos.i > 0 && connects(matrix[pos.i][pos.j], matrix[pos.i - 1][pos.j], 'U');
    let right = pos.j < matrix[pos.i].len() - 1
        && connects(matrix[pos.i][pos.j], matrix[pos.i][pos.j + 1], 'R');
    let down =
        pos.i < matrix.len() - 1 && connects(matrix[pos.i][pos.j], matrix[pos.i + 1][pos.j], 'D');

    if up && down {
        return '|';
    } else if left && right {
        return '-';
    } else if up && right {
        return 'L';
    } else if up && left {
        return 'J';
    } else if left && down {
        return '7';
    } else if down && right {
        return 'F';
    }
    return '.';
}

fn process(input: &str) -> i64 {
    let mut result = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        matrix.push(line.chars().collect());
    }
    let mut visited: Vec<Vec<bool>> = vec![vec![false; matrix[0].len()]; matrix.len()];
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

    matrix[start.i][start.j] = find_start_char(&matrix, &start);

    let mut at = start.clone();
    let mut last = at.clone();
    loop {
        let neighbors = neighbors(&matrix, &at);
        if neighbors.len() == 0 {
            break;
        }
        for n in neighbors.iter() {
            if *n != last {
                visited[at.i][at.j] = true;
                last = at.clone();
                at = n.clone();
                break;
            }
        }
        if at == start {
            break;
        }
    }

    for i in 0..matrix.len() {
        let mut inside = false;
        for j in 0..matrix[i].len() {
            if visited[i][j] {
                if ['|', 'L', 'J'].contains(&matrix[i][j]) {
                    inside = !inside;
                }
            } else if inside {
                result += 1;
            }
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let out = process(input);
        assert_eq!(out, 4);
    }

    #[test]
    fn test2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let out = process(input);
        assert_eq!(out, 8);
    }

    #[test]
    fn test3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let out = process(input);
        assert_eq!(out, 10);
    }
}
