pub fn part_one(input: &str) -> Option<u32> {
    let mut prev: Option<u32> = None;
    let mut count: u32 = 0;
    for line in input.split_whitespace() {
        let number: u32 = line.parse().unwrap();
        count += match prev {
            None => {
                prev = Some(number);
                0
            },
            Some(previous) => {
                let result = if number > previous { 1 } else { 0 };
                prev = Some(number);
                result
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}