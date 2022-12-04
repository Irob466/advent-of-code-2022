use itertools::Itertools;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

// mod rucksack;

/// Type alias for error to allow for ? unrwapping
pub type Error = Box<dyn std::error::Error>;
/// Syntactic sugar for Result and the dynamic error type
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let file = File::open("inputs/day-3.txt")?;
    let reader = BufReader::new(file);

    let priority_sum = reader
        .lines()
        .take_while(|x| x.is_ok())
        .map(|line| line.unwrap())
        .map(|s| duplicates(&s))
        .map(|dups| priority(&dups))
        .sum::<u32>();

    println!("Part 1 answer: {priority_sum}");

    let file = File::open("inputs/day-3.txt")?;
    let reader = BufReader::new(file);

    let priority_sum = reader
        .lines()
        .take_while(|x| x.is_ok())
        .map(|line| line.unwrap())
        .tuples::<(String, String, String)>()
        .map(|w| common_item(w))
        .map(|c| match c {
            Some(item) => priority(&item.to_string()),
            None => 0,
        })
        .sum::<u32>();

    println!("Part 2 answer: {priority_sum}");

    Ok(())
}

fn duplicates(s: &str) -> String {
    let (left, right) = s.split_at(s.len() / 2usize);
    let mut char_set: HashSet<char> = HashSet::new();
    let mut res_set: HashSet<char> = HashSet::new();

    for c in left.chars() {
        char_set.insert(c);
    }

    for c in right.chars() {
        if char_set.contains(&c) {
            res_set.insert(c);
        }
    }

    res_set.iter().collect()
}

fn common_item(sacks: (String, String, String)) -> Option<char> {
    let first_sack_set: HashSet<char> = HashSet::from_iter(sacks.0.chars().into_iter());
    let second_sack_set: HashSet<char> = HashSet::from_iter(sacks.1.chars().into_iter());
    let third_sack_set: HashSet<char> = HashSet::from_iter(sacks.2.chars().into_iter());

    let two_three_intersect = second_sack_set.intersection(&third_sack_set).copied();

    let binding = two_three_intersect.collect();
    let mut intersect = first_sack_set.intersection(&binding);

    intersect.next().copied()
}

fn priority(s: &str) -> u32 {
    s.chars()
        .into_iter()
        .map(|c| c as u32)
        .map(|i| if i >= 97 { i - 96 } else { i - 38 })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::common_item;
    use crate::duplicates;
    use crate::priority;

    #[test]
    fn find_duplicates() {
        assert_eq!(duplicates("vJrwpWtwJgWrhcsFMMfFFhFp"), "p");
        assert_eq!(duplicates("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), "L");
        assert_eq!(duplicates("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), "v");
    }

    #[test]
    fn find_priorities() {
        assert_eq!(priority("L"), 38);
        assert_eq!(priority("v"), 22);
    }

    #[test]
    fn find_common_items() {
        assert_eq!(
            common_item((
                "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                "PmmdzqPrVvPwwTWBwg".to_string()
            )),
            Some('r')
        );
    }

    #[test]
    fn find_common_items_negative() {
        assert_eq!(
            common_item((
                "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                "PmmdzqPVvPwwTWBwg".to_string()
            )),
            None
        );
    }
}
