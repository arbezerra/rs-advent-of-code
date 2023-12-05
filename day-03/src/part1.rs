fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn is_adjacent(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    for k in 0..3 {
        for l in 0..3 {
            if i + k >= 1 && i + k < matrix.len() + 1 && j + l >= 1 && j + l < matrix.len() + 1 {
                let adjacent = matrix[i + k - 1][j + l - 1];

                if adjacent != '.' && !adjacent.is_numeric() {
                    return true;
                }
            }
        }
    }
    return false;
}

fn process(input: &str) -> i32 {
    let mut total = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        matrix.push(line.chars().collect());
    }
    for (i, row) in matrix.iter().enumerate() {
        let mut number = 0;
        let mut adjacent = false;
        for (j, val) in row.iter().enumerate() {
            if val.is_numeric() {
                number = number * 10 + (*val as i32 - 0x30);
                adjacent = adjacent || is_adjacent(&matrix, i, j);
            } else {
                if adjacent {
                    total += number;
                }
                number = 0;
                adjacent = false;
            }
        }
        if adjacent {
            total += number;
        }
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let out = process(input);
        assert_eq!(out, 4361);
    }
}
