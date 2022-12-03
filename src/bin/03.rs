use std::{io::{prelude::*, stdin}};

pub struct Item {
    item_type: char
}

impl Item {
    pub fn new(item_type: char) -> Self {
        Self { item_type }
    }

    fn score(&self) -> usize {
        match self.item_type {
            'a'..='z' => 1 + (self.item_type as usize - 'a' as usize),
            'A'..='Z' => 27 + (self.item_type as usize - 'A' as usize),
            _ => panic!()
        }
    }
}

#[derive(Clone)]
pub struct Rucksack {
    items: String
}

impl Rucksack {
    fn parse_all<R: BufRead>(reader: R) -> Vec<Rucksack> {
        reader.lines()
            .map(|line| Rucksack::new(&line.unwrap()))
            .collect()
    }

    fn new(items: &str) -> Self {
        Self {
            items: items.to_string()
        }
    }

    pub fn left(&self) -> Rucksack {
        let n = self.items.len() / 2;

        Self { items: self.items[..n].to_string() }
    }

    pub fn right(&self) -> Rucksack {
        let n = self.items.len() / 2;

        Self { items: self.items[n..].to_string() }
    }

    pub fn common_items(&self) -> Vec<char> {
        let common = self.left().intersect(&self.right());

        common.items.chars().collect()
    }

    pub fn intersect(&self, other: &Rucksack) -> Rucksack {
        let mut common = self.items
            .chars()
            .filter(|&item| other.items.contains(item))
            .map(|ch| ch as u8)
            .collect::<Vec<_>>();

        common.sort();
        common.dedup();
        Rucksack {
            items: String::from_utf8(common).unwrap()
        }
    }
}

pub struct ElfGroup {
    sacks: Vec<Rucksack>
}

impl ElfGroup {
    fn split_all(sacks: &[Rucksack]) -> Vec<ElfGroup> {
        sacks.chunks_exact(3).map(|chunk| ElfGroup::new(chunk.to_vec())).collect::<Vec<_>>()
    }

    fn new(sacks: Vec<Rucksack>) -> Self {
        Self { sacks }
    }

    pub fn common_items(&self) -> Vec<char> {
        let mut common = self.sacks[0].clone();

        for other_sack in &self.sacks[1..] {
            common = common.intersect(&other_sack);
        }

        common.items.chars().collect()
    }
}

fn main() {
    let stdin = stdin().lock();
    let rucksacks = Rucksack::parse_all(stdin);
    let groups = ElfGroup::split_all(&rucksacks);

    println!("{}", rucksacks.iter().flat_map(|sack| sack.common_items()).map(|item_type| Item::new(item_type).score()).sum::<usize>());
    println!("{}", groups.iter().flat_map(|group| group.common_items()).map(|item_type| Item::new(item_type).score()).sum::<usize>());
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn _01_left_compartment() {
        let rucksack = Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp");

        assert_eq!(rucksack.left().items, "vJrwpWtwJgWr");
    }

    #[test]
    fn _01_right_compartment() {
        let rucksack = Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp");

        assert_eq!(rucksack.right().items, "hcsFMMfFFhFp");
    }

    #[test]
    fn _01_example() {
        const EXAMPLE: &'static str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        let rucksacks = Rucksack::parse_all(Cursor::new(EXAMPLE));

        assert_eq!(rucksacks.len(), 6);
        assert_eq!(rucksacks[0].common_items(), vec! ['p']);
        assert_eq!(rucksacks[1].common_items(), vec! ['L']);
        assert_eq!(rucksacks[2].common_items(), vec! ['P']);
        assert_eq!(rucksacks[3].common_items(), vec! ['v']);
        assert_eq!(rucksacks[4].common_items(), vec! ['t']);
        assert_eq!(rucksacks[5].common_items(), vec! ['s']);
        assert_eq!(rucksacks.iter().flat_map(|sack| sack.common_items()).map(|item_type| Item::new(item_type).score()).sum::<usize>(), 157);
    }

    #[test]
    fn _02_example() {
        const EXAMPLE: &'static str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        let rucksacks = Rucksack::parse_all(Cursor::new(EXAMPLE));
        let groups = ElfGroup::split_all(&rucksacks);

        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].common_items(), vec! ['r']);
        assert_eq!(groups[1].common_items(), vec! ['Z']);
        assert_eq!(groups.iter().flat_map(|group| group.common_items()).map(|item_type| Item::new(item_type).score()).sum::<usize>(), 70);
    }
}
