mod metrics;
mod math_test;
mod errors;
mod time_table_test;

use std::rc::Rc;

use math_test::problem_generator::MultiplyProblemGenerator;
use metrics::metric_writer::MetricWriter;
use time_table_test::TimeTableTest;

fn main() {
    let problem_generator = Rc::new(MultiplyProblemGenerator::new(1,12));
    let metrics = MetricWriter::new();
    let test = TimeTableTest::new(problem_generator, metrics, 10);
    test.run_test();
}
