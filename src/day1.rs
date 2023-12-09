use aho_corasick::{AhoCorasick, PatternID};
use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .split("\n")
        .map(|l| first_and_last(l.as_bytes()))
        .sum::<u32>()
}

fn first_and_last(bytes: &[u8]) -> u32 {
    let mut first = 0;
    let mut last = 0;

    for b in bytes {
        if (b'0'..=b'9').contains(b) {
            if first == 0 {
                first = *b;
            }

            last = *b;
        }
    }

    char_to_num(first) * 10 + char_to_num(last)
}

fn char_to_num(ch: u8) -> u32 {
    (ch - b'0') as u32
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let strings = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let ac = AhoCorasick::new(strings).unwrap();
    input
        .split("\n")
        .map(|l| first_and_last_with_text(&ac, l))
        .sum::<u32>()
}

fn first_and_last_with_text(ac: &AhoCorasick, bytes: &str) -> u32 {
    let mut first = 999;
    let mut last = 0;

    for mat in ac.find_overlapping_iter(bytes) {
        if first == 999 {
            first = mat.pattern().as_u32();
        }
        last = mat.pattern().as_u32();
    }

    (first % 9 + 1) * 10 + last % 9 + 1
}
