mod metrics;
mod math_test;
mod errors;
mod time_table_test;
mod cli_input;

use std::rc::Rc;

use cli_input::{QuizType, UserInput};
use math_test::generators::prepared_problem_generator::PreparedProblemGenerator;
use math_test::problem_generator::ProblemGenerator;

use crate::math_test::generators::random_multiply_problem_generator::RandomMultiplyProblemGenerator;
use crate::metrics::metric_writer::MetricWriter;
use crate::time_table_test::TimeTableTest;

fn main() {
    let user_input = UserInput::get_user_cli_input();
    let metrics = MetricWriter::new();
    let problem_generator: Rc<dyn ProblemGenerator> = match user_input.quiz_type {
            QuizType::Random => Rc::new(RandomMultiplyProblemGenerator::new(1,12)),
            QuizType::Wrongest => Rc::new(PreparedProblemGenerator::new(metrics.get_wrongest_problems())),
            QuizType::Slowest => Rc::new(PreparedProblemGenerator::new(metrics.get_slowest_problems())),
    };
    let test = TimeTableTest::new(problem_generator, metrics, 10);
    test.run_test();
}


