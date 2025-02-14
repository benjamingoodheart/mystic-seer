use rustyline::error::ReadlineError;
use rustyline::{ DefaultEditor, Result };
use rand::random_range;

pub struct MysticSeer<'a> {
    answers: Vec<&'a str>,
}

impl<'a> MysticSeer<'a> {
    pub fn new() -> MysticSeer<'a> {
        MysticSeer {
            answers: vec![
                "Why Not?",
                "It is quite possible",
                "You may never know",
                "If you move soon",
                "That makes a good deal of sense",
                "Try again",
                "There's no question about it",
                "Do you dare risk finding out?",
                "What do you think?",
                "Your chances are good",
                "It has already been taken care of",
                "If that's what you really want",
                "The answer to that is obvious",
                "That's up to you to find out",
                "It all depends on your point of view"
            ],
        }
    }
    pub fn prompt(&self) -> Result<String> {
        let mut rl = rustyline::DefaultEditor::new()?;
        let readline = rl.readline("Put in a penny? Y/N \n >> ")?;
        if readline.trim().to_uppercase() == "Y" {
            println!("You selected yes.");
            println!("{}", self.ask_seer().unwrap());
            println!("{}", self.get_random_answer());
            self.prompt();
        }
        if readline.trim() == "N" {
            println!("You selected no. Just in the nick of time.");
        }
        if readline.trim() != "N" && readline.trim() != "Y" {
            println!("What the hell are you trying to pull here?");
            self.prompt();
        }
        Ok(readline)
    }

    pub fn ask_seer(&self) -> Result<String> {
        let mut rl = rustyline::DefaultEditor::new()?;
        let readline = rl.readline("Ask the mystic seer a yes or no question. \n >>");
        //TODO: Validate answer
        if self.validate_yes_no_question(&readline.as_ref().unwrap()) != true{
            println!("I don't think that's a yes or no question. Try again.");
            self.ask_seer();
        };
        Ok(readline?)
    }

    pub fn get_answer_length(&self) -> usize {
        self.answers.len()
    }

    pub fn get_random_answer(&self) -> String {
        let ind = random_range(0..self.answers.len());
        self.answers[ind].to_string()
    }

    fn validate_yes_no_question(&self, question: &str) -> bool{

        if question.contains("?"){
            match question{
                _ if question.contains("Am")=> {
                    println!("Valid");
                    true
                },
                &_ => todo!(),
            }
        } else{
            println!("Invalid");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MysticSeer;
    #[test]
    fn basics() {
        let seer = MysticSeer::new();
        assert_eq!(seer.answers.len(), 15);

        //TODO: BUILD OUT MORE TESTS
    }
}
