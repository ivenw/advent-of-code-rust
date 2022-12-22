use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: VecDeque<u64>,
    pub operation: Operation,
    pub test: u64,
    pub target_monkey: (u64, u64),
    pub times_inspected: u64,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn apply(&self, item: &u64) -> u64 {
        match self {
            Operation::Add(value) => item + value,
            Operation::Multiply(value) => item * value,
            Operation::Square => item * item,
        }
    }
}

impl Monkey {
    pub fn inspect_item(&mut self) -> u64 {
        self.times_inspected += 1;
        let item = self.items.pop_front().unwrap();

        (self.operation.apply(&item) as f64 / 3.0).floor() as u64
    }

    pub fn inspect_item_part_2(&mut self, lcm: &u64) -> u64 {
        self.times_inspected += 1;
        let item = self.items.pop_front().unwrap();

        self.operation.apply(&item) % lcm
    }

    pub fn throw_item(&mut self, item: u64) -> usize {
        if item % self.test == 0 {
            self.target_monkey.0 as usize
        } else {
            self.target_monkey.1 as usize
        }
    }
}
