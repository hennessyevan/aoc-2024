advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    for line in input.lines() {
        let mut line_numbers = line
            .chars()
            .filter(|c| char::is_numeric(*c))
            .collect::<Vec<_>>();

        if line_numbers.len() == 1 {
            line_numbers.push(line_numbers.first().unwrap().clone());
        }

        if line_numbers.len() > 2 {
            line_numbers = vec![
                *line_numbers.first().unwrap(),
                *line_numbers.last().unwrap(),
            ];
        }

        let number = line_numbers
            .iter()
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        numbers.push(number);
    }

    let result = numbers.iter().sum::<u32>().into();

    return result;
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
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
