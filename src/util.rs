pub fn parse_num_fast(val: &[u8]) -> u32 {
    match *val {
        [a] => char_to_num(a),
        [a, b] => 10 * char_to_num(a) + char_to_num(b),
        _ => 0,
    }
}

pub fn char_to_num(ch: u8) -> u32 {
    (ch - b'0') as u32
}
