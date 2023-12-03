use std::{io, ops::Range};

fn main() {
    let mut prev: Option<String> = None;
    let mut lines = io::stdin().lines().peekable();
    let mut sum = 0;
    while let Some(current) = lines.next() {
        let current = current.unwrap();
        let next = lines.peek().map(|l| l.as_deref().unwrap());
        process_line(&mut sum, &current, prev.as_ref().map(|s| s.as_str()), next);
        prev = Some(current);
    }
    println!("{sum}");
}

fn process_line(sum: &mut usize, curr: &str, prev: Option<&str>, next: Option<&str>) {
    let curr_sym = symbol_indices(curr);
    let prev_sym = prev.map(symbol_indices);
    let next_sym = next.map(symbol_indices);

    let mut num_start = Some(0usize);
    let mut num = 0usize;
    for (i, &c) in curr.as_bytes().iter().enumerate() {
        if c.is_ascii_digit() {
            if num_start.is_none() {
                num_start = Some(i);
            }
            num = num * 10 + (c - b'0') as usize;
        } else if let Some(i0) = num_start {
            if is_adjacent(i0..i, Some(&curr_sym))
                || is_adjacent(i0..i, prev_sym.as_ref())
                || is_adjacent(i0..i, next_sym.as_ref())
            {
                *sum += num;
            }
            num = 0;
            num_start = None;
        }
    }
    if let Some(i0) = num_start {
        if is_adjacent(i0..curr.len(), Some(&curr_sym))
            || is_adjacent(i0..curr.len(), prev_sym.as_ref())
            || is_adjacent(i0..curr.len(), next_sym.as_ref())
        {
            *sum += num;
        }
    }
}

fn is_adjacent(index_range: Range<usize>, syms: Option<&Vec<bool>>) -> bool {
    let Some(syms) = syms else {
        return false;
    };
    let start = index_range.start.saturating_sub(1);
    let end = (index_range.end + 1).min(syms.len());
    syms[start..end].iter().any(|&sym| sym)
}

fn symbol_indices(line: &str) -> Vec<bool> {
    line.as_bytes()
        .iter()
        .map(|&c| !c.is_ascii_digit() && c != b'.')
        .collect()
}
