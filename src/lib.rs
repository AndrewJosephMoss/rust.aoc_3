use itertools::Itertools;

pub fn process_part_1(input: &str) -> usize {
    let common_items = get_common_compartment_items(&input);
    let total_prior = common_items.iter().map(|item| get_char_priority(*item).unwrap()).sum();
    total_prior
}

pub fn proccess_part_2(input: &str) -> usize {
    let badge_items = input.lines().chunks(3).into_iter().map(|grp| get_common_grp_item(&grp.collect::<Vec<&str>>())).collect::<Vec<char>>();
    let total_prior = badge_items.iter().map(|item| get_char_priority(*item).unwrap()).sum::<usize>();
    total_prior
}

fn get_common_grp_item(member_items: &Vec<&str>) -> char {
    let common_item: char = member_items[0].chars().find(|c| member_items[1].contains(*c) && member_items[2].contains(*c)).unwrap();
    common_item
}

fn get_common_compartment_items(input: &str) -> Vec<char> {
    input.lines().map(|sack| {
        let sack_size = sack.len() / 2;
        let compartment_a = &sack[..sack_size];
        let compartment_b = &sack[sack_size..];
        let common_item = compartment_a.chars().find(|c| compartment_b.contains(*c)).unwrap();
        common_item
    }).collect::<Vec<char>>()
}

fn get_char_priority(c: char) -> Option<usize> {
    let letter_priorities = ('a'..='z').chain('A'..='Z');
    let priority = letter_priorities.enumerate().find_map(|(idx, ch)| {
        if ch == c {
            Some(idx + 1)
        } else {
            None
        }
    });
    priority
}

#[cfg(test)]
mod test {
    use std::fs;
    use super::*;

    #[test]
    fn test_part_2() {
        let test_input2 = fs::read_to_string("test-input2.txt").unwrap();
        let total_prior = proccess_part_2(&test_input2);
        assert_eq!(total_prior, 70);
    }

    #[test]
    fn test_get_common_grp_item() {
        let grp1_input = fs::read_to_string("test-group1-input.txt").unwrap();
        let member_items = grp1_input.lines().collect::<Vec<&str>>();
        let common_item = get_common_grp_item(&member_items);
        assert_eq!(common_item, 'r');

        let grp2_input = fs::read_to_string("test-group2-input.txt").unwrap();
        let member_items = grp2_input.lines().collect::<Vec<&str>>();
        let common_item = get_common_grp_item(&member_items);
        assert_eq!(common_item, 'Z');
    }

    #[test]
    fn test_get_common_items() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let common_items = get_common_compartment_items(&input);
        assert_eq!(common_items, ['p', 'L', 'P', 'v', 't', 's']);
    }

    #[test]
    fn test_get_priority() {
        let prior = get_char_priority('a').unwrap();
        assert_eq!(prior, 1);
        let prior = get_char_priority('z').unwrap();
        assert_eq!(prior, 26);
        let prior = get_char_priority('A').unwrap();
        assert_eq!(prior, 27);
        let prior = get_char_priority('Z').unwrap();
        assert_eq!(prior, 52);
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let result = process_part_1(&input);
        assert_eq!(result, 157);
    }
}