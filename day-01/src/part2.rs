use std::collections::HashMap;

#[derive(Debug)]
struct Value {
    index: i32,
    value: i32,
}

fn main() {
    let input = include_str!("./part1.txt");
    let output = process(input);
    println!("{}", output);
}

fn create_map() -> HashMap<&'static str, i32> {
    let mut map: HashMap<&'static str, i32> = HashMap::new();
    map.insert("one", 1);
    map.insert("1", 1);
    map.insert("two", 2);
    map.insert("2", 2);
    map.insert("three", 3);
    map.insert("3", 3);
    map.insert("four", 4);
    map.insert("4", 4);
    map.insert("five", 5);
    map.insert("5", 5);
    map.insert("six", 6);
    map.insert("6", 6);
    map.insert("seven", 7);
    map.insert("7", 7);
    map.insert("eight", 8);
    map.insert("8", 8);
    map.insert("nine", 9);
    map.insert("9", 9);

    return map;
}

fn process(input: &str) -> String {
    let map = create_map();
    let mut total = 0;
    for line in input.lines() {
        let mut first = Value {
            index: -1,
            value: 0,
        };
        let mut last = Value {
            index: -1,
            value: 0,
        };

        for (key, val) in &map {
            let first_i = match line.find(key){
                Some(x) => x as i32,
                None => -1,
            };
            if first_i != -1 && (first.index == -1 || first_i < first.index){
                first.index = first_i as i32;
                first.value = *val;
            }
            let last_i = match line.rfind(key){
                Some(x) => x as i32,
                None => -1,
            };
            if last_i != -1 && (last.index == -1 || last_i > last.index){
                last.index = last_i as i32;
                last.value = *val;
            }
        }


        dbg!(&first);
        dbg!(&last);
        total += 10 * first.value + last.value;
        dbg!(total);
    }
    return total.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let out = process(input);
        assert_eq!(out, "281");
    }
}
