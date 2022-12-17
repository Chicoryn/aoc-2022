use ndarray::{Array, Array2, s, stack, Axis, concatenate, ArrayView2};
use std::{io::{stdin, BufRead}, collections::HashMap};

trait Shape {
    fn starting_point(&self) -> Array2<i8>;
}

struct Line;
struct Plus;
struct L;
struct I;
struct O;

/// `####`
impl Shape for Line {
    fn starting_point(&self) -> Array2<i8> {
        stack(Axis(0), &[
            Array::from_vec(vec! [0, 0, 1, 1, 1, 1, 0]).view(),
        ]).unwrap()
    }
}

/// ```
/// .#.
/// ###
/// .#.
/// ```
impl Shape for Plus {
    fn starting_point(&self) -> Array2<i8> {
        stack(Axis(0), &[
            Array::from_vec(vec! [0, 0, 0, 1, 0, 0, 0]).view(),
            Array::from_vec(vec! [0, 0, 1, 1, 1, 0, 0]).view(),
            Array::from_vec(vec! [0, 0, 0, 1, 0, 0, 0]).view(),
        ]).unwrap()
    }
}

/// ```
/// ..#
/// ..#
/// ###
/// ```
impl Shape for L {
    fn starting_point(&self) -> Array2<i8> {
        stack(Axis(0), &[
            Array::from_vec(vec! [0, 0, 1, 1, 1, 0, 0]).view(),
            Array::from_vec(vec! [0, 0, 0, 0, 1, 0, 0]).view(),
            Array::from_vec(vec! [0, 0, 0, 0, 1, 0, 0]).view(),
        ]).unwrap()
    }
}

/// ```
/// #
/// #
/// #
/// #
/// ```
impl Shape for I {
    fn starting_point(&self) -> Array2<i8> {
        stack(Axis(0), &[
            Array::from_vec(vec! [0, 0, 1, 0, 0, 0, 0]).view(),
            Array::from_vec(vec! [0, 0, 1, 0, 0, 0, 0]).view(),
            Array::from_vec(vec! [0, 0, 1, 0, 0, 0, 0]).view(),
            Array::from_vec(vec! [0, 0, 1, 0, 0, 0, 0]).view(),
        ]).unwrap()
    }
}

/// ```
/// ##
/// ##
/// ```
impl Shape for O {
    fn starting_point(&self) -> Array2<i8> {
        stack(Axis(0), &[
            Array::from_vec(vec! [0, 0, 1, 1, 0, 0, 0]).view(),
            Array::from_vec(vec! [0, 0, 1, 1, 0, 0, 0]).view(),
        ]).unwrap()
    }
}

fn rocks() -> [Box<dyn Shape>; 5] {
    [
        Box::new(Line {}),
        Box::new(Plus {}),
        Box::new(L {}),
        Box::new(I {}),
        Box::new(O {}),
    ]
}

fn try_push_left(rock: &Array2<i8>) -> Array2<i8>{
    if rock.slice(s! [.., 0]).sum() > 0 {
        rock.clone()
    } else {
        concatenate(Axis(1), &[
            rock.slice(s! [.., 1..]),
            Array::from_elem((rock.dim().0, 1), 0i8).view(),
        ]).unwrap()
    }
}

fn try_push_right(rock: &Array2<i8>) -> Array2<i8>{
    if rock.slice(s! [.., 6]).sum() > 0 {
        rock.clone()
    } else {
        concatenate(Axis(1), &[
            Array::from_elem((rock.dim().0, 1), 0i8).view(),
            rock.slice(s! [.., ..6]),
        ]).unwrap()
    }
}

fn intersects_at(chamber: ArrayView2<'_, i8>, rock: ArrayView2<'_, i8>, y: usize) -> bool {
    for (i, lane) in rock.lanes(Axis(1)).into_iter().enumerate() {
        let y = y + i;

        if y < chamber.dim().0 {
            if (&chamber.row(y) + &lane).iter().any(|&s| s > 1) {
                return true;
            }
        }
    }

    false
}

fn fall_rock (
    mut chamber: Array2<i8>,
    mut rock: Array2<i8>,
    jet_stream_seq: &mut impl Iterator<Item=char>
) -> (Array2<i8>, usize)
{
    let mut y = chamber.dim().0 + 3;
    let mut steps = 0;

    loop {
        if let Some(wind)= jet_stream_seq.next() {
            let moved_rock = match wind {
                '<' => try_push_left(&rock),
                '>' => try_push_right(&rock),
                _ => panic!()
            };

            if !intersects_at(chamber.view(), moved_rock.view(), y) {
                rock = moved_rock;
            }
        }

        steps += 1;
        if !intersects_at(chamber.view(), rock.view(), y - 1) {
            y -= 1;
        } else {
            break
        }
    }

    if chamber.dim().0 < (y + rock.dim().0) {
        chamber = concatenate(Axis(0), &[
            chamber.view(),
            Array::from_elem((y + rock.dim().0 - chamber.dim().0, 7), 0i8).view(),
        ]).unwrap();
    }

    let mut affected_lanes = chamber.slice_mut(s! [
        y..(y+rock.dim().0),
        ..
    ]);

    affected_lanes += &rock;
    (chamber, steps)
}

// num_rounds: usize
fn play_aux<T>(
    mut until_fn: impl FnMut(ArrayView2<i8>, usize, usize, usize) -> Option<T>,
    starting_chamber: Option<Array2<i8>>,
    rock_index: usize,
    jet_stream_seq: &[char],
    mut jet_stream_index: usize
) -> Option<T>
{
    let mut chamber = starting_chamber.unwrap_or(Array2::from_elem((1, 7), 1i8));
    let mut steps = 0;

    for (rock_i, rock) in rocks().iter().enumerate().cycle().skip(rock_index) {
        if let Some(x) = until_fn(chamber.view(), steps, rock_i, jet_stream_index) {
            return Some(x)
        }

        let (next_chamber, jet_stream_steps) = fall_rock(
            chamber,
            rock.starting_point(),
            &mut jet_stream_seq.iter().cloned().cycle().skip(jet_stream_index)
        );

        debug_assert!(next_chamber.iter().any(|&x| x <= 1), "{}", next_chamber);

        jet_stream_index = (jet_stream_index + jet_stream_steps) % jet_stream_seq.len();
        chamber = next_chamber;
        steps += 1;
    }

    None
}

fn play(num_rounds: usize, jet_stream_seq: &[char]) -> usize {
    // when playing with large `num_rounds` it the play ground should eventually
    // look like this:
    //
    // ```
    // [garbage]
    // [cycle]
    // [cycle]
    // [cycle]
    // [garbage]
    // ```
    //
    // We need to figure out the cycles, how they interlock, and what the start
    // and end garbage looks like.
    //
    let mut visited = HashMap::new();
    let (after_cycle, jet_stream_cycle_at, rocks_cycle_at, cycle_step_length, cycle_height, start_garbage_height, start_garbage_steps) = play_aux(move |chamber, i, rock_index, jet_stream_seq| {
        if i >= num_rounds {
            Some(None)
        } else if chamber.dim().0 >= 10 {
            let contour = chamber.slice(s! [
                (chamber.dim().0 - 10)..,
                ..
            ]);

            if visited.contains_key(&(rock_index, jet_stream_seq, contour.to_owned())) {
                let (cycle_start_step, cycle_start_height) = visited[&(rock_index, jet_stream_seq, contour.to_owned())];
                let (cycle_end_step, cycle_end_height) = (i, chamber.dim().0);
                let cycle_step_length = cycle_end_step - cycle_start_step;
                let cycle_height = cycle_end_height - cycle_start_height;
                let start_garbage_height = cycle_start_height;
                let start_garbage_steps = cycle_start_step;

                Some(Some((chamber.to_owned(), jet_stream_seq, rock_index, cycle_step_length, cycle_height, start_garbage_height, start_garbage_steps)))
            } else {
                visited.insert((rock_index, jet_stream_seq, contour.to_owned()), (i, chamber.dim().0));
                None
            }
        } else {
            None
        }
    }, None, 0, jet_stream_seq, 0).unwrap().unwrap();

    // figure out how many garbage lines we have at the end of the cycles
    let num_cycles = (num_rounds - start_garbage_steps) / cycle_step_length;
    let end_garbage_steps = num_rounds - start_garbage_steps - num_cycles * cycle_step_length;
    let after_cycle_garbage = play_aux(|chamber, i, _, _| {
        if i >= end_garbage_steps {
            Some(chamber.to_owned())
        } else {
            None
        }
    }, Some(after_cycle.to_owned()), rocks_cycle_at, jet_stream_seq, jet_stream_cycle_at).unwrap();
    let end_garbage_height = after_cycle_garbage.dim().0 - after_cycle.dim().0;

    start_garbage_height
        + num_cycles * cycle_height
        + end_garbage_height
        - 1
}

fn main() {
    let mut jet_stream_seq = String::new();
    stdin().lock().read_line(&mut jet_stream_seq).unwrap();

    println!("{}", play(2022, &jet_stream_seq.chars().collect::<Vec<_>>()));
    println!("{}", play(1000000000000, &jet_stream_seq.chars().collect::<Vec<_>>()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn _01_falling_rocks() {
        let sequence = EXAMPLE.chars().collect::<Vec<_>>();
        let chamber = play_aux(|chamber, i, _, _| {
            if i >= 10 {
                Some(chamber.to_owned())
            } else {
                None
            }
        }, None, 0, &sequence, 0).unwrap();

        assert_eq!(chamber, stack(Axis(0), &[
            Array::from_vec(vec! [1, 1, 1, 1, 1, 1, 1]).view(),
            Array::from_vec(vec! [0, 0, 1, 1, 1, 1, 0]).view(),
            Array::from_vec(vec! [0, 0, 0, 1, 0, 0, 0]).view(),
            Array::from_vec(vec! [0, 0, 1, 1, 1, 0, 0]).view(),
            Array::from_vec(vec! [1, 1, 1, 1, 1, 0, 0]).view(),
            Array::from_vec(vec! [0, 0, 1, 0, 1, 0, 0]).view(),
            Array::from_vec(vec! [0, 0, 1, 0, 1, 0, 0]).view(),
            Array::from_vec(vec! [0, 0, 0, 0, 1, 0, 0]).view(),
            Array::from_vec(vec! [0, 0, 0, 0, 1, 1, 0]).view(),
            Array::from_vec(vec! [0, 0, 0, 0, 1, 1, 0]).view(),
            Array::from_vec(vec! [0, 1, 1, 1, 1, 0, 0]).view(),
            Array::from_vec(vec! [0, 0, 1, 0, 0, 0, 0]).view(),
            Array::from_vec(vec! [0, 1, 1, 1, 0, 0, 0]).view(),
            Array::from_vec(vec! [1, 1, 1, 1, 1, 1, 0]).view(),
            Array::from_vec(vec! [1, 1, 0, 0, 1, 1, 0]).view(),
            Array::from_vec(vec! [0, 0, 0, 0, 1, 1, 0]).view(),
            Array::from_vec(vec! [0, 0, 0, 0, 1, 0, 0]).view(),
            Array::from_vec(vec! [0, 0, 0, 0, 1, 0, 0]).view(),
        ]).unwrap());
    }

    #[test]
    fn _01_example() {
        let sequence = EXAMPLE.chars().collect::<Vec<_>>();
        assert_eq!(play(2022, &sequence), 3068);
    }

    #[test]
    fn _02_example() {
        let sequence = EXAMPLE.chars().collect::<Vec<_>>();
        assert_eq!(play(1000000000000, &sequence), 1514285714288);
    }
}