use std::hash::{Hash, Hasher};

use priority_queue::PriorityQueue;

fn main() {
    let mut exp = Expedition::default();
    aoc::read_from_stdin(&mut exp);
    let mut total: usize = 0;
    for _ in 1..=3 {
        total += exp.pop_highest();
    }
    println!("{}", total);
}

#[derive(Default, Debug)]
struct Expedition {
    cur_elf: Option<Elf>,
    pq: PriorityQueue<Elf, usize>,
}

impl Expedition {
    fn pop_highest(&mut self) -> usize {
        self.pq.pop().unwrap().1
    }
}

#[derive(Default, Debug)]
struct Backpack {
    items: Vec<usize>,
}

#[derive(Debug)]
struct Elf {
    num: usize,
    backpack: Backpack,
    total: usize,
}

impl Elf {
    fn new(num: usize) -> Self {
        Elf {
            num,
            backpack: Default::default(),
            total: Default::default(),
        }
    }
}

impl Hash for Elf {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.num.hash(state)
    }
}

impl Eq for Elf {}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl aoc::LineParser for Expedition {
    fn parse_line(&mut self, line: &str) {
        if self.cur_elf.is_none() {
            let _ = self.cur_elf.insert(Elf::new(self.pq.len()));
        }

        if line.len() == 0 {
            let elf = self.cur_elf.take().unwrap();
            let cal = elf.total;
            self.pq.push(elf, cal);
            return;
        }
        let calories = line.parse().unwrap();

        let elf = self.cur_elf.as_mut().unwrap();
        elf.backpack.items.push(calories);
        elf.total += calories;
    }
}
