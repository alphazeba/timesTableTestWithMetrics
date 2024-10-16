use std::cell::RefCell;
use rand::{rngs::ThreadRng, Rng};

use crate::math_test::constants::Int;
pub struct Randy {
    min_value: Int,
    max_value: Int,
    generator: RefCell<ThreadRng>,
}

impl Randy {
    pub fn new(min_value: Int, max_value: Int) -> Self {
        Self {
            min_value,
            max_value,
            generator: RefCell::new(rand::thread_rng()),
        }
    }

    pub fn get_random_value(&self) -> Int {
        self.generator.borrow_mut().gen_range(self.min_value..(self.max_value+1))
    }

    pub fn get_random_value_but_prefer_not_1(&self) -> Int {
        match self.get_random_value() {
            1 => self.get_random_value(),
            x => x,
        }
    }

    pub fn get_50_50(&self) -> bool {
        self.generator.borrow_mut().gen_bool(0.5)
    }
}