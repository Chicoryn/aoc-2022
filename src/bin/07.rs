use sscanf::sscanf;
use std::io::{prelude::*, stdin};
use std::collections::HashMap;

enum FsEntry {
    Directory(FsDirectory),
    File { size: usize }
}

impl FsEntry {
    fn is_dir(&self) -> bool {
        match &self {
            Self::File { .. } => false,
            Self::Directory(_) => true
        }
    }

    fn size(&self) -> usize {
        match &self {
            Self::File { size } => *size,
            Self::Directory(dir) => dir.size()
        }
    }

    fn traverse<R>(&self, f: &impl Fn(R, &FsEntry) -> R, initial_value: R) -> R {
        match &self {
            Self::File { .. } => initial_value,
            Self::Directory(dir) => dir.traverse(f, initial_value)
        }
    }
}

struct FsDirectory {
    entries: HashMap<String, FsEntry>
}

impl FsDirectory {
    fn empty() -> Self {
        Self { entries: HashMap::new() }
    }

    fn get_directory_mut(&mut self, name: &String) -> &mut FsDirectory {
        match self.entries.get_mut(name) {
            Some(FsEntry::Directory(dir_entry)) => dir_entry,
            _ => { panic!() }
        }
    }

    fn insert(&mut self, name: String, entry: FsEntry) {
        self.entries.insert(name, entry);
    }

    fn size(&self) -> usize {
        self.entries.values().map(|entry| entry.size()).sum()
    }

    fn traverse<R>(&self, f: &impl Fn(R, &FsEntry) -> R, mut initial_value: R) -> R {
        for entry in self.entries.values() {
            initial_value = f(initial_value, entry);
        }

        initial_value
    }
}

struct FsConsumer {
    root: FsDirectory,
    current_path: Vec<String>
}

impl FsConsumer {
    fn parse_all<R: BufRead>(reader: R) -> Self {
        let mut consumer = Self::new();

        for line in reader.lines().filter_map(|line| line.ok()) {
            consumer.consume(line);
        }

        consumer
    }

    fn new() -> Self {
        Self {
            root: FsDirectory::empty(),
            current_path: vec! [],
        }
    }

    fn root(&self) -> &FsDirectory {
        &self.root
    }

    fn get_current_directory(&mut self) -> &mut FsDirectory {
        let mut current = &mut self.root;

        for directory_name in &self.current_path {
            current = current.get_directory_mut(&directory_name);
        }

        current
    }

    fn traverse<R>(&self, f: impl Fn(R, &FsEntry) -> R, initial_value: R) -> R {
        self.root().traverse(&f, initial_value)
    }

    fn consume(&mut self, line: String) {
        if let Ok(_) = sscanf!(line, "$ cd /") {
            self.current_path.clear();
        } else if let Ok(_) = sscanf!(line, "$ cd ..") {
            self.current_path.pop();
        } else if let Ok(new_directory) = sscanf!(line, "$ cd {}", String) {
            self.current_path.push(new_directory);
        } else if let Ok(_) = sscanf!(line, "$ ls") {
            // pass
        } else if let Ok(directory_name) = sscanf!(line, "dir {}", String) {
            self.get_current_directory().insert(directory_name, FsEntry::Directory(FsDirectory::empty()));
        } else if let Ok((size, file_name)) = sscanf!(line, "{} {}", usize, String) {
            self.get_current_directory().insert(file_name, FsEntry::File { size });
        } else {
            panic!("could not parse line -- {}", line);
        }
    }
}

fn sum_of_at_most_100000(mut acc: usize, entry: &FsEntry) -> usize {
    let entry_size = entry.size();

    if entry.is_dir() && entry_size <= 100000 {
        acc += entry_size;
    }

    entry.traverse(&sum_of_at_most_100000, acc)
}

fn smallest_bigger_than(limit: usize) -> impl Fn(usize, &FsEntry) -> usize {
    move |acc, entry| {
        let entry_size = entry.size();

        if entry.is_dir() && entry_size >= limit {
            entry.traverse(&smallest_bigger_than(limit), acc.min(entry_size))
        } else {
            acc
        }
    }
}

fn main() {
    let stdin = stdin().lock();
    let consumer = FsConsumer::parse_all(stdin);
    let total_disk_space = 70000000;
    let needed_free_space = 30000000;
    let space_to_free_up = needed_free_space - (total_disk_space - consumer.root().size());

    println!("{}", consumer.traverse(sum_of_at_most_100000, 0));
    println!("{}", consumer.traverse(smallest_bigger_than(space_to_free_up), usize::MAX));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn _01_example() {
        let consumer = FsConsumer::parse_all(Cursor::new(EXAMPLE));

        assert_eq!(consumer.root().size(), 48381165);
        assert_eq!(consumer.traverse(sum_of_at_most_100000, 0), 95437);
    }

    #[test]
    fn _02_example() {
        let consumer = FsConsumer::parse_all(Cursor::new(EXAMPLE));
        let total_disk_space = 70000000;
        let needed_free_space = 30000000;
        let space_to_free_up = needed_free_space - (total_disk_space - consumer.root().size());

        assert_eq!(space_to_free_up, 8381165);
        assert_eq!(consumer.traverse(smallest_bigger_than(space_to_free_up), usize::MAX), 24933642);
    }
}
