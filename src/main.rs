mod metrics;
mod math_test;

use math_test::problem_generator::{self, MultiplyProblemGenerator, ProblemGenerator};
use metrics::metric_writer::metricWriter;

fn main() {
    println!("Hello, world!");
    metricWriter::test();

    let problem_generator = MultiplyProblemGenerator::new(1,10);
    question_loop(&problem_generator);
}

fn question_loop(problem_generator: &dyn ProblemGenerator) {
    let problem = problem_generator.generate_problem();

}