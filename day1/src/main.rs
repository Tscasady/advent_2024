use std::{collections::HashMap, fs, str::Lines};

fn main() {
    let result = part_one()
    println!("Part one: {result}");

    let result = part_two()
    println!("Part two: {result}");

}


fn part_one() -> u32 {
    let text = fs::read_to_string("day1/input.txt").unwrap();
    let lines = text.lines();
    let mut nums1: Vec<u32> = vec![];
    let mut nums2: Vec<u32> = vec![];
    lines.for_each(|line| binary_insert_line(parse_numbers(line), &mut nums1, &mut nums2));
    total_distance(nums1, nums2)
}

fn parse_numbers(line: &str) -> (u32, u32) {
    let chunks = line.split(" ").collect::<Vec<_>>();
    let left = chunks.first().unwrap().parse::<u32>().unwrap();
    let right = chunks.last().unwrap().parse::<u32>().unwrap();
    (left, right)
}

fn binary_insert_line(nums: (u32, u32), nums1: &mut Vec<u32>, nums2: &mut Vec<u32>) {
    let (left, right) = nums;
    let left_pos = nums1.binary_search(&left).unwrap_or_else(|index| index);
    let right_pos = nums2.binary_search(&right).unwrap_or_else(|index| index);
    nums1.insert(left_pos, left);
    nums2.insert(right_pos, right);
}

fn total_distance(mut nums1: Vec<u32>, mut nums2: Vec<u32>) -> u32 {
    let mut total = 0;
    while let Some(num) = nums1.pop() {
        while let Some(num2) = nums2.pop() {
            total += num.abs_diff(num2);
            break;
        }
    }
    total
}

pub fn part_two() -> u32 {
    let text = fs::read_to_string("day1/input.txt").unwrap();
    let lines = text.lines();
    let (left, right) = nums_to_maps(lines);
    let mut total = 0;
    for (key, value) in left {
        let result = right.get(&key).unwrap_or(&0);
        total += key * result * value
    }
    total
}

fn nums_to_maps(lines: Lines) -> (HashMap<u32, u32>, HashMap<u32, u32>) {
    let mut left_map = HashMap::new();
    let mut right_map = HashMap::new();
    for line in lines {
        let (left, right) = parse_numbers(line);
        left_map.entry(left).and_modify(|counter| *counter +=1).or_insert(1);

        right_map.entry(right).and_modify(|counter| *counter += 1).or_insert(1);
    }
    (left_map, right_map)
}

mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(11, part_one())
    }

    #[test]
    fn test_part_two() {
        assert_eq!(31, part_two())
    }
}
