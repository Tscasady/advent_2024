use std::fs;

pub fn part_one() -> u32 {
    let text = fs::read_to_string("day1/input.txt").unwrap();
    let lines = text.lines();
    let mut nums1: Vec<u32> = vec![];
    let mut nums2: Vec<u32> = vec![];
    lines.for_each(|line| parse_numbers(line, &mut nums1, &mut nums2));
    // println!("{:?}", nums1);
    // println!("{:?}", nums2);
    total_distance(nums1, nums2)
}

fn parse_numbers(line: &str, nums1: &mut Vec<u32>, nums2: &mut Vec<u32>) {
    let chunks = line.split(" ").collect::<Vec<_>>();
    let left = chunks.first().unwrap().parse::<u32>().unwrap();
    let right = chunks.last().unwrap().parse::<u32>().unwrap();
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
            break
        }
    }
    total
}

mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(11, part_one())
    }
}
