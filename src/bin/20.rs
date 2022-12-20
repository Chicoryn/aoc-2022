use std::{io::{prelude::*, stdin}, iter};

#[derive(Clone)]
struct Num {
    value: i64,
    next: usize,
    prev: usize
}

struct Mixer {
    buf: Vec<i64>
}

impl Mixer {
    fn parse_all(reader: impl BufRead) -> Self {
        Mixer {
            buf: reader.lines()
                .filter_map(|line| line.ok())
                .map(|line| line.parse::<i64>().unwrap())
                .collect()
        }
    }

    fn scale(&self, scalar: i64) -> Self {
        Self {
            buf: self.buf.iter().map(|&value| value * scalar).collect()
        }
    }

    fn mix(&self, num_mixes: usize) -> Self {
        // construct a linked list
        let mut ll = vec! [];

        for &value in &self.buf {
            if ll.is_empty() {
                ll.push(Num {
                    value,
                    next: 0,
                    prev: 0
                });
            } else {
                let new_index = ll.len();
                ll.first_mut().unwrap().prev = new_index;
                ll.last_mut().unwrap().next = new_index;

                ll.push(Num {
                    value,
                    next: 0,
                    prev: new_index - 1
                });
            }
        }

        // rotate the linked list
        let n = ll.len();

        for _ in 0..num_mixes {
            for i in 0..n {
                let mut rot = (ll[i].value.abs() % ((n as i64) - 1)) * ll[i].value.signum();

                // rotate right
                while rot > 0 {
                    let next = ll[i].next;
                    let next_next = ll[next].next;
                    let prev = ll[i].prev;

                    ll[prev].next = next;
                    ll[next].prev = prev;
                    ll[next].next = i;
                    ll[next_next].prev = i;
                    ll[i].prev = next;
                    ll[i].next = next_next;
                    rot -= 1;
                }

                // rotate left
                while rot < 0 {
                    let next = ll[i].next;
                    let prev = ll[i].prev;
                    let prev_prev = ll[prev].prev;

                    ll[next].prev = prev;
                    ll[prev].next = next;
                    ll[prev].prev = i;
                    ll[prev_prev].next = i;
                    ll[i].prev = prev_prev;
                    ll[i].next = prev;
                    rot += 1;
                }
            }
        }

        // re-read the ordered linked list
        let zero = self.buf.iter().position(|&x| x == 0).unwrap();
        let mut current = Some(zero);
        let buf = iter::from_fn(move || {
                if let Some(curr) = current {
                    if ll[curr].next == zero {
                        current = None;
                    } else {
                        current = Some(ll[curr].next);
                    }

                    Some(ll[curr].value)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Self { buf }
    }

    fn at(&self, index: usize) -> i64 {
        self.buf[index % self.buf.len()]
    }
}

fn main() {
    let stdin = stdin().lock();
    let mix = Mixer::parse_all(stdin);
    let mix1 = mix.mix(1);
    println!("{}", [1000, 2000, 3000].into_iter().map(|i| mix1.at(i)).sum::<i64>()); // 11123

    let mix10 = mix.scale(811589153).mix(10);
    println!("{}", [1000, 2000, 3000].into_iter().map(|i| mix10.at(i)).sum::<i64>()); // 4248669215955
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE: &str = r#"1
2
-3
3
-2
0
4"#;

    #[test]
    fn _01_example() {
        let mix = Mixer::parse_all(Cursor::new(EXAMPLE)).mix(1);
        assert_eq!(mix.buf, vec! [0, 3, -2, 1, 2, -3, 4]);
        assert_eq!([1000, 2000, 3000].into_iter().map(|i| mix.at(i)).sum::<i64>(), 3);
    }

    #[test]
    fn _02_example() {
        let mix = Mixer::parse_all(Cursor::new(EXAMPLE)).scale(811589153);
        assert_eq!(mix.buf,         vec! [811589153, 1623178306, -2434767459, 2434767459, -1623178306, 0, 3246356612]);
        assert_eq!(mix.mix( 1).buf, vec! [0, -2434767459, 3246356612, -1623178306, 2434767459, 1623178306, 811589153]);
        assert_eq!(mix.mix( 2).buf, vec! [0, 2434767459, 1623178306, 3246356612, -2434767459, -1623178306, 811589153]);
        assert_eq!(mix.mix( 3).buf, vec! [0, 811589153, 2434767459, 3246356612, 1623178306, -1623178306, -2434767459]);
        assert_eq!(mix.mix( 4).buf, vec! [0, 1623178306, -2434767459, 811589153, 2434767459, 3246356612, -1623178306]);
        assert_eq!(mix.mix( 5).buf, vec! [0, 811589153, -1623178306, 1623178306, -2434767459, 3246356612, 2434767459]);
        assert_eq!(mix.mix( 6).buf, vec! [0, 811589153, -1623178306, 3246356612, -2434767459, 1623178306, 2434767459]);
        assert_eq!(mix.mix( 7).buf, vec! [0, -2434767459, 2434767459, 1623178306, -1623178306, 811589153, 3246356612]);
        assert_eq!(mix.mix( 8).buf, vec! [0, 1623178306, 3246356612, 811589153, -2434767459, 2434767459, -1623178306]);
        assert_eq!(mix.mix( 9).buf, vec! [0, 811589153, 1623178306, -2434767459, 3246356612, 2434767459, -1623178306]);
        assert_eq!(mix.mix(10).buf, vec! [0, -2434767459, 1623178306, 3246356612, -1623178306, 2434767459, 811589153]);
        assert_eq!([1000, 2000, 3000].into_iter().map(|i| mix.mix(10).at(i)).sum::<i64>(), 1623178306);
    }
}
