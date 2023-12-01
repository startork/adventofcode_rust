advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let number_map: Vec<&str> = input.lines().collect();
    let numbers: Vec<String> = number_map
    .iter()
    .map(|&line| line.chars().find(|&c| c.is_numeric()).unwrap_or_default().to_string() + &line.chars().rev().find(|&c| c.is_numeric()).unwrap_or_default().to_string() )
    .collect();
    print!("{:?}", numbers);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
