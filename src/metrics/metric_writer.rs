use rusqlite::{params, Connection, Row, Rows};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::math_test::{constants::Int, problem::Problem};

pub struct MetricWriter {
    db: Connection,
    test_id: String,
}
type ProblemTypeType = u8;
const METRICS_DB_NAME: &str = "timeTableTestMetrics.db";
const PROBLEM_TYPE_MULTIPLICATION: ProblemTypeType = 0;

pub struct MetricInstance {
    start_time: OffsetDateTime,
    num_wrong: u32,
}

impl MetricInstance {
    pub fn new() -> Self {
        Self {
            start_time: OffsetDateTime::now_utc(),
            num_wrong: 0,
        }
    }

    pub fn add_wrong(&mut self) {
        self.num_wrong += 1;
    }
}

impl MetricWriter {
    pub fn new() -> Self {
        let path = METRICS_DB_NAME.to_string();
        let err_msg = format!("Could not open db conn with {}", path);
        let db = Connection::open(path).expect(&err_msg);
        db.execute(
            "create table if not exists timeTableMetrics \
            (test_id TEXT not null, timestamp DATE not null, duration INTEGER not null, \
            a INTEGER not null, b INTEGER not null, num_incorrect INTEGER not null, problem_type INTEGER not null)",
            []).expect("something went wrong with the metrics table");
        Self {
            db,
            test_id: generate_new_test_id(),
        }
    }

    pub fn metric(&self, problem: Problem, metric_instance: MetricInstance) {
        let start_time = metric_instance.start_time.unix_timestamp();
        let duration: i64 = (OffsetDateTime::now_utc() - metric_instance.start_time)
            .whole_milliseconds()
            .try_into()
            .expect("failed to convert duration to i64");
        let a = problem.get_lower();
        let b = problem.get_higher();
        let num_incorrect = metric_instance.num_wrong;
        let problem_type = PROBLEM_TYPE_MULTIPLICATION;
        match self.db.execute(
            "insert into timeTableMetrics \
            (test_id, timestamp, duration, a, b, num_incorrect, problem_type) \
            values \
            (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![self.test_id, start_time, duration, a, b, num_incorrect, problem_type]
        ) {
            Ok(_updated) => (),
            Err(err) => println!("update failed: {}", err),
        }
    }

    pub fn get_wrongest_problems(&self) -> Vec<Problem> {
        let mut statement = self.db.prepare(
            "select a, b, problem_type, avg(num_incorrect) as wrong \
            from timeTableMetrics \
            group by a, b, problem_type \
            order by wrong desc \
            limit 10").expect("preparing wrongest query failed");
        Self::parse_query_results(
            statement.query([])
            .expect("binding params failed, but there shouldn't be params"))
    }

    pub fn get_slowest_problems(&self) -> Vec<Problem> {
        let mut statement = self.db.prepare(
            "select a, b, problem_type, avg(duration) as wrong \
            from timeTableMetrics \
            group by a, b, problem_type \
            order by wrong desc \
            limit 10").expect("preparing wrongest query failed");
        Self::parse_query_results(
            statement.query([])
            .expect("binding params failed, but there shouldn't be params"))
    }

    fn parse_query_results(mut query_result: Rows) -> Vec<Problem> {
        let mut problems: Vec<Problem> = Vec::new();
        loop {
            match query_result.next() {
                Err(e) => panic!("failed query!: {}", e),
                Ok(thing) => match thing {
                    Some(thing) => problems.push(Self::parse_row(thing)),
                    None => break,
                },
            }
        }
        problems
    }

    fn parse_row(row: &Row) -> Problem {
        let a: Int = row.get(0).expect("couldn't parse a");
        let b: Int = row.get(1).expect("couldn't parse b");
        let problem_type: ProblemTypeType = row.get(2).expect("couldn't parse problem_type");
        Problem::new(format!("{} * {}", a, b), a*b, a, b)
    }
}

fn generate_new_test_id() -> String {
    Uuid::new_v4().to_string()
}
