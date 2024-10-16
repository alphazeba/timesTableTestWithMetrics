use std::cmp::{max, min};

use crate::math_test::constants::Int;

#[derive(Clone)]
pub struct Problem {
    answer: Int,
    question: String,
    a: Int,
    b: Int,
}

impl Problem {
    pub fn new(question: String, answer: Int, a: Int, b: Int) -> Self {
        Self {
            question,
            answer,
            a,b,
        }
    }

    pub fn get_question(&self) -> String {
        self.question.clone()
    }

    pub fn check_answer(&self, guess: Int) -> bool {
        self.answer == guess
    }

    pub fn swap_a_b(&mut self) {
        let temp = self.a;
        self.a = self.b;
        self.b = temp;
        self.question = format!("{} * {}", self.a, self.b);
    }

    pub fn get_lower(&self) -> Int {
        min(self.a, self.b)
    }
    pub fn get_higher(&self) -> Int {
        max(self.a, self.b)
    }
}