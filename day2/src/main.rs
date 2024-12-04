use std::{env, fs, path::PathBuf};

fn main() {
    let result = part_one("./day2/input.txt".into());
    println!("Part one: {result}");

    let result = part_two("./day2/input.txt".into());
    println!("Part two: {result}");
}

fn part_one(path: PathBuf) -> usize {
    let text = fs::read_to_string(path).unwrap();
    let lines = text.lines();
    lines
        .filter(|line| {
            let nums = to_level(line);
            (is_increasing(&nums) || is_decreasing(&nums)) && is_safe(&nums)
        })
        .count()
}

fn part_two(path: PathBuf) -> usize {
    let text = fs::read_to_string(path).unwrap();
    let lines = text.lines();
    lines
        .filter(|line| {
            let nums = to_level(line);
            let len = nums.len();
            (is_mostly_increasing(&nums, len) || is_mostly_decreasing(&nums, len)) && is_mostly_safe(&nums, len)
        })
        .count()
}

fn to_level(line: &str) -> Vec<i32> {
    line.split(" ")
        .filter_map(|chunk| chunk.parse::<i32>().ok())
        .collect::<Vec<_>>()
}

fn is_increasing(nums: &[i32]) -> bool {
    nums.windows(2).all(|x| x[0] < x[1])
}

fn is_decreasing(nums: &[i32]) -> bool {
    nums.windows(2).all(|x| x[0] > x[1])
}

fn is_safe(nums: &[i32]) -> bool {
    nums.windows(2).all(|x| ((x[0]-x[1]).abs() >= 1) && ((x[0]-x[1]).abs() < 4))
}

fn is_mostly_increasing(nums: &[i32], len: usize) -> bool {
    nums.windows(2).filter(|x| x[0] < x[1]).count() >= len - 1 
}

fn is_mostly_decreasing(nums: &[i32], len: usize) -> bool {
    nums.windows(2).filter(|x| x[0] > x[1]).count() >= len - 1
}

fn is_mostly_safe(nums: &[i32], len: usize) -> bool {
    nums.windows(2).filter(|x| ((x[0]-x[1]).abs() >= 1) && ((x[0]-x[1]).abs() < 4)).count() <= len - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(2, part_one("./test.txt".into()))
    }

    #[test]
    fn test_part_two() {
        assert_eq!(4, part_two("./test.txt".into()))
    }

}

