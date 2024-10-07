use rusqlite::{params, Connection};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::math_test::problem::Problem;

pub struct MetricWriter {
    db: Connection,
    test_id: String,
}

const METRICS_DB_NAME: &str = "timeTableTestMetrics.db";
const PROBLEM_TYPE_MULTIPLICATION: u8 = 0;

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
}

fn generate_new_test_id() -> String {
    Uuid::new_v4().to_string()
}
