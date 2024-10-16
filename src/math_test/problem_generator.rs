use crate::math_test::problem::Problem;

pub trait ProblemGenerator {
    fn generate_problem(&self) -> Problem;
}
