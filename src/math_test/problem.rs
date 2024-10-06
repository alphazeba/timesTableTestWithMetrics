use crate::math_test::constants::Int;
pub struct Problem {
    answer: Int,
    question: String,
}

impl Problem {
    pub fn new(question: String, answer: Int) -> Self {
        Self {
            question,
            answer,
        }
    }

    pub fn get_question(&self) -> String {
        self.question.clone()
    }

    pub fn check_answer(&self, guess: Int) -> bool {
        self.answer == guess
    }
}