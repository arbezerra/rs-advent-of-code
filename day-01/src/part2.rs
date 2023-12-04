fn main() {
    let input = include_str!("./part1.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
        let mut first = -1;
        let mut last = -1;
        for c in line.chars() {
            if c.is_numeric() {
                if first == -1 {
                    first = c as i32 - 0x30;
                } else {
                    last = c as i32 - 0x30;
                }
                dbg!(first);
                dbg!(last);
            }
        }
        if last == -1 {
            last = first
        }

        total += 10 * first + last;
        dbg!(total);
    }
    return total.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let out = process(input);
        assert_eq!(out, "142");
    }
}

