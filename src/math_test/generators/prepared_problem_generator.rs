use crate::math_test::{constants::Int, problem::Problem, problem_generator::ProblemGenerator, randy::Randy};

pub struct PreparedProblemGenerator {
    problem_list: Vec<Problem>,
    randy: Randy,
}

impl PreparedProblemGenerator {
    pub fn new(problems: Vec<Problem>) -> Self {
        let problem_length: Int = problems.len().try_into().unwrap();
        Self {
            problem_list: problems,
            randy: Randy::new(
                0,
                problem_length-1,
            ),
        }
    }
}

impl ProblemGenerator for PreparedProblemGenerator {
    fn generate_problem(&self) -> Problem {
        let random_index: usize = self.randy.get_random_value().try_into().unwrap();
        let mut random_problem = self.problem_list
            .get(random_index)
            .unwrap()
            .clone();
        // problems have a,b sorted when stored, so randomize their order on retrieval.
        if self.randy.get_50_50() {
            random_problem.swap_a_b();
        }
        random_problem
    }
}