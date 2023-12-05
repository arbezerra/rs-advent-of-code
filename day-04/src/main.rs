use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> i32 {
    let mut total: i32 = 0;
    for line in input.lines(){
        let mut result: i32 = -1;
        let mut winner: HashSet<i32> = HashSet::new();

        let game: Vec<&str> = line.split(": ").collect();
        let parts: Vec<&str> = game[1].split(" | ").collect();

        for wn in parts[0].split_whitespace().map(|x| x.parse::<i32>().unwrap()){
            winner.insert(wn);
        }
        for n in parts[1].split_whitespace().map(|x| x.parse::<i32>().unwrap()){
            if winner.contains(&n){
                result +=1;
            }
        }

        if result > -1{
            total += i32::pow(2, TryFrom::try_from(result).unwrap());
        }
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let out = process(input);
        assert_eq!(out, 13);
    }
}

