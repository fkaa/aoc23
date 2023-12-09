use aho_corasick::{AhoCorasick, PatternID};
use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| parse_line(l))
        .filter_map(|l| l)
        .map(|g| g.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    input.lines().map(|l| parse_line2(l)).sum()
}

#[derive(Debug)]
struct Game {
    id: u32,
    blues: u32,
    reds: u32,
    greens: u32,
}
fn parse_line(line: &str) -> Option<Game> {
    let (_, line) = line.split_once(' ').unwrap();
    let (id, line) = line.split_once(": ").unwrap();

    let id = id.parse::<u32>().unwrap();

    let mut blues = 0;
    let mut reds = 0;
    let mut greens = 0;

    for set in line.split("; ") {
        for col in set.split(", ") {
            let (count, col) = col.split_once(' ').unwrap();
            let count = parse_num_fast(count.as_bytes());
            match col.len() {
                3 => {
                    if count > 12 {
                        return None;
                    }
                    reds += count;
                }
                4 => {
                    if count > 14 {
                        return None;
                    }
                    blues += count;
                }
                5 => {
                    if count > 13 {
                        return None;
                    }
                    greens += count;
                }
                _ => {}
            }
            // println!("{set} => {col}");
        }
    }

    let g = Game {
        id,
        blues,
        reds,
        greens,
    };
    Some(g)
}

fn parse_line2(line: &str) -> u32 {
    let (_, line) = line.split_once(' ').unwrap();
    let (id, line) = line.split_once(": ").unwrap();

    let id = id.parse::<u32>().unwrap();

    let mut blues = 0;
    let mut reds = 0;
    let mut greens = 0;

    for set in line.split("; ") {
        for col in set.split(", ") {
            let (count, col) = col.split_once(' ').unwrap();
            let count = parse_num_fast(count.as_bytes());
            match col.len() {
                3 => {
                    reds = reds.max(count);
                }
                4 => {
                    blues = blues.max(count);
                }
                5 => {
                    greens = greens.max(count);
                }
                _ => {}
            }
        }
    }

    blues * reds * greens
}

pub fn parse_num_fast(val: &[u8]) -> u32 {
    match *val {
        [a] => char_to_num(a),
        [a, b] => 10 * char_to_num(a) + char_to_num(b),
        _ => unreachable!(),
    }
}

pub fn char_to_num(ch: u8) -> u32 {
    (ch - b'0') as u32
}
