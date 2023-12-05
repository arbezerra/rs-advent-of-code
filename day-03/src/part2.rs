fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn get_element(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
    let mut k = j;
    while k > 0 && matrix[i][k].is_ascii_digit() {
        k -= 1;
    }
    if !matrix[i][k].is_ascii_digit(){
        k+=1;
    }
    let mut number: i64 = 0;
    while k < matrix[i].len() && matrix[i][k].is_ascii_digit() {
        number = number * 10 + (matrix[i][k] as i64 - 0x30);
        k += 1;
    }
    return number;
}

fn get_adjacents(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    if i >= 1 {
        if matrix[i - 1][j].is_ascii_digit() {
            result.push(get_element(&matrix, i - 1, j));
        } else {
            if j >= 1 && matrix[i - 1][j - 1].is_ascii_digit() {
                result.push(get_element(&matrix, i - 1, j - 1));
            }
            if j + 1 < matrix[i].len() && matrix[i - 1][j + 1].is_ascii_digit() {
                result.push(get_element(&matrix, i - 1, j + 1));
            }
        }
    }
    if j >= 1 && matrix[i][j - 1].is_ascii_digit() {
        result.push(get_element(&matrix, i, j - 1));
    }
    if j + 1 < matrix[i].len() && matrix[i][j + 1].is_ascii_digit() {
        result.push(get_element(&matrix, i, j + 1));
    }

    if i + 1 < matrix.len() {
        if matrix[i + 1][j].is_ascii_digit() {
            result.push(get_element(&matrix, i + 1, j));
        } else {
            if j >= 1 && matrix[i + 1][j - 1].is_ascii_digit() {
                result.push(get_element(&matrix, i + 1, j - 1));
            }
            if j + 1 < matrix[i].len() && matrix[i + 1][j + 1].is_ascii_digit() {
                result.push(get_element(&matrix, i + 1, j + 1));
            }
        }
    }

    return result;
}

fn process(input: &str) -> i64 {
    let mut total: i64 = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        matrix.push(line.chars().collect());
    }
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '*' {
                let adjacents = get_adjacents(&matrix, i, j);
                if adjacents.len() == 2 {
                    total += adjacents[0] * adjacents[1];
                }
            }
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
        assert_eq!(out, 467835);
    }
}
