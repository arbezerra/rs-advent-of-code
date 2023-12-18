fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let time: Vec<_> = lines[0].split(":").collect();
    let time = time[1].replace(" ", "").parse::<f64>().unwrap();
    let distance: Vec<_> = lines[1].split(":").collect();
    let distance = distance[1].replace(" ", "").parse::<f64>().unwrap();

    let delta = time.powi(2) - 4.0 * (distance + 1.0);
    let x1 = (time - f64::from(delta).sqrt()) / 2.0;
    let x2 = (time + f64::from(delta).sqrt()) / 2.0;

    return (x2.floor() - x1.ceil()) as i32 + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let out = process(input);
        assert_eq!(out, 71503);
    }
}
