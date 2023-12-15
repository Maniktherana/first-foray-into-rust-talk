fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut res = 0;
    for line in input.lines() {
        let line = line.replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        // convert lines to list of chars
        let chars: Vec<char> = line.chars().collect();
        // remove all non-numeric chars
        let nums: Vec<char> = chars.into_iter().filter(|c| c.is_numeric()).collect();
        // get first and last items and convert to i32
        let first = nums.first().unwrap().to_digit(10).unwrap() as i32;
        let last = nums.last().unwrap().to_digit(10).unwrap() as i32;
        let num = first * 10 + last;

        res += num;
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2() {
        let result = part2("two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen");
        assert_eq!(result, 281);
    }
}