//! A rucksack is a collection of items in two compartments. A single type of item cannot be in both compartments.

use std::{collections::HashSet, str::FromStr};

use crate::{Error, Result};
pub struct Rucksack<'a>((&'a str, &'a str));

impl Rucksack<'_> {
    /// Find duplicated items in the rucksack.
    fn duplicates(&self) -> String {
        let mut char_set: HashSet<char> = HashSet::new();
        let mut res = String::new();

        for c in self.0 .0.chars() {
            char_set.insert(c);
        }

        for c in self.0 .1.chars() {
            if char_set.contains(&c) {
                res.push_str(&c.to_string());
            }
        }

        res
    }

    fn priority(&self) -> u8 {
        todo!()
    }
}

impl FromStr for Rucksack<'_> {
    type Err = Error;

    fn from_str<'a>(s: &'a str) -> Result<Self> {
        let compartments = s.split_at(s.len() / 2usize);
        Ok(Rucksack(compartments))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Rucksack;

    #[test]
    fn find_duplicates() {
        let sack = Rucksack::from_str("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap();
        assert_eq!(sack.duplicates(), "L");
        let sack = Rucksack::from_str("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").unwrap();
        assert_eq!(sack.duplicates(), "v");
    }

    #[test]
    fn find_priorities() {
        let sack = Rucksack::from_str("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap();
        assert_eq!(sack.priority(), 38);
        let sack = Rucksack::from_str("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").unwrap();
        assert_eq!(sack.priority(), 22);
    }
}
