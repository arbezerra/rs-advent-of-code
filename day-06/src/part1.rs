fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> i32 {
    let mut result = 1;
    let lines: Vec<&str> = input.lines().collect();

    let time: Vec<_> = lines[0].split_whitespace().collect();
    let time: Vec<_> = time[1..]
        .into_iter()
        .map(|e| e.parse::<f64>().unwrap())
        .collect();
    let distance: Vec<_> = lines[1].split_whitespace().collect();
    let distance: Vec<_> = distance[1..]
        .into_iter()
        .map(|e| e.parse::<f64>().unwrap())
        .collect();

    for i in 0..time.len() {
        let delta = time[i].powi(2) - 4.0 * (distance[i] + 1.0);
        let x1 = (time[i] - f64::from(delta).sqrt()) / 2.0;
        let x2 = (time[i] + f64::from(delta).sqrt()) / 2.0;
        result *= (x2.floor() - x1.ceil()) as i32 + 1;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let out = process(input);
        assert_eq!(out, 288);
    }
}
