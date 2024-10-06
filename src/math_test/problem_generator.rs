use std::cell::RefCell;
use rand::{rngs::ThreadRng, Rng};

use crate::math_test::constants::Int;
use crate::math_test::problem::Problem;

pub trait ProblemGenerator {
    fn generate_problem(&self) -> Problem;
}

pub struct MultiplyProblemGenerator {
    randy: Randy,
}

impl MultiplyProblemGenerator {
    pub fn new(min_value: Int, max_value: Int) -> Self {
        Self {
            randy: Randy::new(min_value, max_value),
        }
    }
}

impl ProblemGenerator for MultiplyProblemGenerator {
    fn generate_problem(&self) -> Problem {
        let a = self.randy.get_random_value_but_prefer_not_1();
        let b = self.randy.get_random_value_but_prefer_not_1();
        let question = format!("{} * {} = ?", a, b);
        Problem::new(question, a*b)
    }
}

struct Randy {
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
}