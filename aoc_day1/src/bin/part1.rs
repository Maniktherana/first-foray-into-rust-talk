fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut res = 0;
    for line in input.lines() {
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
    fn test1() {
        let result = part1("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, 142);
    }
}
