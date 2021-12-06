extern crate aoc2021;
use aoc2021::shared;

fn generate_sonar(content: &String) -> Vec<u32> {
    content
        .split('\n')
        .map(|x| match x.trim().parse::<u32>() {
            Err(why) => panic!("couldn't parse {}: {}", x, why),
            Ok(value) => value,
        })
        .collect()
}

// How many values that are greater than previous value?
fn part1(sonar: &Vec<u32>) -> u32 {
    let mut count: u32 = 0;
    for i in 1..sonar.len() {
        if sonar[i] > sonar[i - 1] {
            count += 1;
        }
    }
    count
}

// How many triplets ("sliding window") with sum greater than the previous triplet?
fn part2(sonar: &Vec<u32>) -> u32 {
    let mut prev: u32 = 0;
    let mut count: u32 = 0;
    for i in 2..sonar.len() {
        let sum3 = sonar[i - 2] + sonar[i - 1] + sonar[i];
        if prev != 0 && sum3 > prev {
            count += 1;
        }
        prev = sum3
    }
    count
}

fn main() {
    let content = shared::read_file("days/01/input");
    let sonar = generate_sonar(&content);
    println!("Part 1: {}", part1(&sonar));
    println!("Part 2: {}", part2(&sonar));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let content = shared::read_file("days/01/input.test");
        let sonar = generate_sonar(&content);
        assert_eq!(part1(&sonar), 7);
    }

    #[test]
    fn test_part2() {
        let content = shared::read_file("days/01/input.test");
        let sonar = generate_sonar(&content);
        assert_eq!(part2(&sonar), 5);
    }
}