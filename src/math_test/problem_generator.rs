use std::cell::RefCell;
use rand::{rngs::ThreadRng, Rng};

use crate::math_test::constants::Int;
use crate::math_test::problem::Problem;

pub trait ProblemGenerator {
    fn generate_problem(&self) -> Problem;
}

pub struct MultiplyProblemGenerator {
    min_value: Int,
    max_value: Int,
    randy: RefCell<ThreadRng>,
}

impl MultiplyProblemGenerator {
    pub fn new(min_value: Int, max_value: Int) -> Self {
        Self {
            min_value,
            max_value,
            randy: RefCell::new(rand::thread_rng()),
        }
    }

    fn get_random_value(&self) -> Int {
        self.randy.borrow_mut().gen_range(self.min_value..(self.max_value+1))
    }
}

impl ProblemGenerator for MultiplyProblemGenerator {
    fn generate_problem(&self) -> Problem {
        let a = self.get_random_value();
        let b = self.get_random_value();
        let question = format!("{} * {} = ?", a, b);
        Problem::new(question, a*b)
    }
}