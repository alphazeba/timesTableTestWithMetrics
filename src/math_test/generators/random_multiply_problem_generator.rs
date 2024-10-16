use crate::math_test::{constants::Int, problem::Problem, problem_generator::ProblemGenerator, randy::Randy};
pub struct RandomMultiplyProblemGenerator {
    randy: Randy,
}

impl RandomMultiplyProblemGenerator {
    pub fn new(min_value: Int, max_value: Int) -> Self {
        Self {
            randy: Randy::new(min_value, max_value),
        }
    }
}

impl ProblemGenerator for RandomMultiplyProblemGenerator {
    fn generate_problem(&self) -> Problem {
        let a = self.randy.get_random_value_but_prefer_not_1();
        let b = self.randy.get_random_value_but_prefer_not_1();
        let question = format!("{} * {} = ?", a, b);
        Problem::new(question, a*b, a, b)
    }
}