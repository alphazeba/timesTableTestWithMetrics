use std::rc::Rc;

use text_io::try_read;

use crate::{errors::InvalidInput, math_test::{constants::Int, problem::Problem, problem_generator::ProblemGenerator}};

pub struct TimeTableTest {
    problem_generator: Rc<dyn ProblemGenerator>,
    num_questions: u8,
}

impl TimeTableTest {
    pub fn new(
        problem_generator: Rc<dyn ProblemGenerator>,
        num_questions: u8
    ) -> Self {
        Self {
            problem_generator,
            num_questions,
        }
    }

    pub fn run_test(&self) {
        println!("Beginning test");
        for i in 0..self.num_questions {
            println!("\nQuestion {}", i+1);
            self.problem_cycle();
        }
        println!("Test complete");
    }

    fn problem_cycle(&self) {
        let problem = self.problem_generator.generate_problem();
        loop {
            let answer = get_player_answer(&problem);
            match problem.check_answer(answer) {
                false => println!("NOOOO!"),
                true => {
                    println!("correct!");
                    break;
                },
            }
        }
    }
}

fn get_player_answer(problem: &Problem) -> Int {
    let mut input_answer: Option<Int> = None;
    while let None = input_answer {
        println!("Question: {}", problem.get_question());
        input_answer = try_read!()
            .map_err(|e| InvalidInput::InputMustBeInt(e))
            .map_err(|e| println!("Invalid input: {:?}", e))
            .ok();
    }
    input_answer.expect("we have validated that input_answer is not none")
}
