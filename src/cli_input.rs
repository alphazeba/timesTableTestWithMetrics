use std::str::FromStr;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliInput {
    #[arg(short, long, default_value="random")]
    quiz_type: String,
}

pub enum QuizType {
    Random,
    Wrongest,
    Slowest,
}

impl FromStr for QuizType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "random" => Ok(QuizType::Random),
            "wrongest" => Ok(QuizType::Wrongest),
            "slowest" => Ok(QuizType::Slowest),
            _ => Err(()),
        }
    }
}

pub struct UserInput {
    pub quiz_type: QuizType,
}

impl UserInput {
    pub fn get_user_cli_input() -> Self {
        let input = CliInput::parse();
        Self {
            quiz_type: QuizType::from_str(
                input.quiz_type.as_str()).expect("invalid quiz type"),
        }
    }
}
