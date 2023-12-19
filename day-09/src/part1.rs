fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process_line(line: &str) -> i64 {
    let mut matrix: Vec<Vec<i64>> = Vec::new();
    matrix.push(line.split_whitespace().map(|e| e.parse::<i64>().unwrap()).collect());

    let mut result = 0;
    loop {
        let builder = matrix.last().unwrap();
        result += builder.last().unwrap();
        let mut new: Vec<_> = Vec::new();
        let mut all_zeros = true;
        for i in 1..builder.len(){
            let diff =builder[i] - builder[i-1];
            all_zeros = all_zeros && diff == 0;
            new.push(builder[i] - builder[i-1]);
        }
        matrix.push(new.clone());
        if all_zeros{
            break;
        }
    }

    return result;
}

fn process(input: &str) -> i64 {
    let mut result = 0;
    for line in input.lines(){
        result += process_line(line);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let out = process(input);
        assert_eq!(out, 114);
    }
}

