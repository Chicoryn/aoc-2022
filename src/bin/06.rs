use std::io::{prelude::*, stdin};

struct DataStreamBuffer {
    characters: Vec<char>
}

impl DataStreamBuffer {
    fn new(buf: &str) -> Self {
        Self {
            characters: buf.chars().collect()
        }
    }

    fn len(&self) -> usize {
        self.characters.len()
    }

    fn is_distinct_sequence(&self, index: usize, size: usize) -> bool {
        let start = index.saturating_sub(size);
        let mut received = self.characters[start..index].to_vec();
        received.sort_unstable();
        received.dedup();

        received.len() == size
    }

    fn is_start_of_packet(&self, index: usize) -> bool {
        self.is_distinct_sequence(index, 4)
    }

    fn is_start_of_message(&self, index: usize) -> bool {
        self.is_distinct_sequence(index, 14)
    }
}

fn main() {
    if let Some(Ok(line)) =stdin().lock().lines().next() {
        let buf = DataStreamBuffer::new(&line);

        println!("{}", (0..buf.len()).filter(|&i| buf.is_start_of_packet(i)).next().unwrap());
        println!("{}", (0..buf.len()).filter(|&i| buf.is_start_of_message(i)).next().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _01_mjqjpqmgbljsphdztnvjfqwrcgsmlb() {
        const EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let buf = DataStreamBuffer::new(EXAMPLE);

        assert_eq!((0..buf.len()).filter(|&i| buf.is_start_of_packet(i)).next(), Some(7));
    }

    #[test]
    fn _01_bvwbjplbgvbhsrlpgdmjqwftvncz() {
        const EXAMPLE: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let buf = DataStreamBuffer::new(EXAMPLE);

        assert_eq!((0..buf.len()).filter(|&i| buf.is_start_of_packet(i)).next(), Some(5));
    }

    #[test]
    fn _01_nppdvjthqldpwncqszvftbrmjlhg() {
        const EXAMPLE: &str = "nppdvjthqldpwncqszvftbrmjlhg";
        let buf = DataStreamBuffer::new(EXAMPLE);

        assert_eq!((0..buf.len()).filter(|&i| buf.is_start_of_packet(i)).next(), Some(6));
    }

    #[test]
    fn _01_nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg() {
        const EXAMPLE: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let buf = DataStreamBuffer::new(EXAMPLE);

        assert_eq!((0..buf.len()).filter(|&i| buf.is_start_of_packet(i)).next(), Some(10));
    }

    #[test]
    fn _01_zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw() {
        const EXAMPLE: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let buf = DataStreamBuffer::new(EXAMPLE);

        assert_eq!((0..buf.len()).filter(|&i| buf.is_start_of_packet(i)).next(), Some(11));
    }

    #[test]
    fn _02_mjqjpqmgbljsphdztnvjfqwrcgsmlb() {
        const EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let buf = DataStreamBuffer::new(EXAMPLE);

        assert_eq!((0..buf.len()).filter(|&i| buf.is_start_of_message(i)).next(), Some(19));
    }

    #[test]
    fn _02_bvwbjplbgvbhsrlpgdmjqwftvncz() {
        const EXAMPLE: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let buf = DataStreamBuffer::new(EXAMPLE);

        assert_eq!((0..buf.len()).filter(|&i| buf.is_start_of_message(i)).next(), Some(23));
    }

    #[test]
    fn _02_nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg() {
        const EXAMPLE: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let buf = DataStreamBuffer::new(EXAMPLE);

        assert_eq!((0..buf.len()).filter(|&i| buf.is_start_of_message(i)).next(), Some(29));
    }

    #[test]
    fn _02_nppdvjthqldpwncqszvftbrmjlhg() {
        const EXAMPLE: &str = "nppdvjthqldpwncqszvftbrmjlhg";
        let buf = DataStreamBuffer::new(EXAMPLE);

        assert_eq!((0..buf.len()).filter(|&i| buf.is_start_of_message(i)).next(), Some(23));
    }

    #[test]
    fn _02_zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw() {
        const EXAMPLE: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let buf = DataStreamBuffer::new(EXAMPLE);

        assert_eq!((0..buf.len()).filter(|&i| buf.is_start_of_message(i)).next(), Some(26));
    }
}
