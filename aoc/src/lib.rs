pub mod pc;

use std::{io, ops::Range};

pub trait LineParser {
    fn start(&mut self) {}
    fn parse_line(&mut self, line: &str);
    fn finish(&mut self) {}
}

pub fn read_from_stdin<T: LineParser>(parser: &mut T) {
    let mut line = String::new();
    loop {
        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                parser.finish();
                break;
            }
            Ok(_) => parser.parse_line(&line.trim()),
            Err(e) => {
                println!("Err: {}", e);
                break;
            }
        }
        line.clear();
    }
}

pub fn grid() -> (Vec<Vec<u8>>, usize, usize) {
    let grid = io::stdin()
        .lines()
        .map(|line| line.unwrap().as_bytes().to_owned())
        .collect::<Vec<_>>();
    let rows = grid.len();
    if let Some(row0) = grid.get(0) {
        let cols = row0.len();
        (grid, rows, cols)
    } else {
        Default::default()
    }
}

pub fn to_range(i: usize, margin: usize, max_i: usize) -> Range<usize> {
    let start = i.saturating_sub(margin);
    let end = i.saturating_add(1 + margin).min(max_i);
    start..end
}

pub fn scan_num_at(src: &[u8], index: usize) -> Option<(usize, Range<usize>)> {
    let mut start = index;
    let mut end = start + 1;
    let mut num = 0;
    if src[index].is_ascii_digit() {
        for s in (0..index).rev() {
            if src[s].is_ascii_digit() {
                start -= 1;
            }
        }
        for e in (index + 1)..src.len() {
            if src[e].is_ascii_digit() {
                end += 1;
            }
        }
        for d in start..end {
            num = num * 10 + (src[d] - b'0') as usize;
        }
        Some((num, start..end))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_range() {
        assert_eq!(0..3, to_range(0, 2, 10));
        assert_eq!(0..4, to_range(1, 2, 10));
        assert_eq!(0..5, to_range(2, 2, 10));
        assert_eq!(1..6, to_range(3, 2, 10));
        assert_eq!(7..10, to_range(8, 1, 10));
        assert_eq!(8..10, to_range(9, 1, 10));
    }

    #[test]
    fn test_scan_num_at() {
        assert_eq!(None, scan_num_at(b"....123...", 3));
        assert_eq!(Some((123, 4..7)), scan_num_at(b"....123...", 4));
        assert_eq!(Some((123, 4..7)), scan_num_at(b"....123...", 5));
        assert_eq!(Some((123, 4..7)), scan_num_at(b"....123...", 6));
        assert_eq!(None, scan_num_at(b"....123...", 7));
    }
}
