use rustyline::{ DefaultEditor, Result };
use rand::random_range;
use std::process;
use colored::{ Colorize };

pub struct MysticSeer<'a> {
    pub answers: Vec<&'a str>,
}

impl<'a> MysticSeer<'a> {
    pub fn new() -> MysticSeer<'a> {
        MysticSeer {
            answers: vec![
                "Why not?",
                "It is quite possible.",
                "You may never know.",
                "If you move soon.",
                "That makes a good deal of sense.",
                "Try again.",
                "There's no question about it.",
                "Do you dare risk finding out?",
                "What do you think?",
                "Your chances are good.",
                "It has already been taken care of.",
                "If that's what you really want.",
                "The answer to that is obvious.",
                "That's up to you to find out.",
                "It all depends on your point of view."
            ],
        }
    }
    pub fn welcome(&self) -> () {
        println!(
            "{}",
            "Welcome to the Busy Bee Cafe. The mystic seer beckons...".purple().italic()
        );
    }

    pub fn prompt(&self) -> Result<String> {
        let mut rl = DefaultEditor::new()?;
        let readline = rl.readline("Put in a penny? Y/N \n >> ")?;
        if readline.trim().to_uppercase() == "Y" {
            println!("{}", "You selected yes.".purple().italic());

            let _ = self.ask_seer();

            println!("{}", self.get_random_answer().magenta().bold());

            let _ = self.prompt();
        }
        if readline.trim().to_uppercase() == "N" {
            println!("{}", "You selected no. Just in the nick of time.".green().italic());
        }
        if readline.trim().to_uppercase() != "N" && readline.trim().to_uppercase() != "Y" {
            println!("{}", "That's not an acceptible answer.".red());
            let _ = self.prompt();
        }
        Ok(readline)
    }

    pub fn ask_seer(&self) -> Result<String> {
        let mut rl = rustyline::DefaultEditor::new()?;
        let readline = rl.readline("Ask the mystic seer a yes or no question. \n >> ");
        if &readline.as_ref().unwrap().to_uppercase() == "N" {
            println!("{}", "Come back again real soon!".green().italic());
            process::exit(0);
        }
        if self.validate_yes_no_question(&readline.as_ref().unwrap()) != true {
            println!(
                "{} {} {}",
                "Hm, I don't know if that's a yes/no question. Try starting it with the words".magenta(),
                " 
            -> Am
            -> Could
            -> Can 
            -> Will
            -> Does
            -> Do
            -> Is
            -> May".dimmed(),
                "
            \n and ensure the question ends with '?' 
            Use 'N' to exit.".purple()
            );
            let _ = self.ask_seer();
        }
        Ok(readline?)
    }

    pub fn get_answer_length(&self) -> usize {
        self.answers.len()
    }

    pub fn get_random_answer(&self) -> String {
        let ind = random_range(0..self.answers.len());
        self.answers[ind].to_string()
    }

    fn validate_yes_no_question(&self, question: &str) -> bool {
        if question.ends_with("?") {
            match question {
                _ if
                    question.to_uppercase().trim().starts_with("AM") |
                    question.to_uppercase().trim().starts_with("COULD") |
                    question.to_uppercase().trim().starts_with("CAN") |
                    question.to_uppercase().trim().starts_with("DOES") |
                    question.to_uppercase().trim().starts_with("WILL") |
                    question.to_uppercase().trim().starts_with("IS") |
                    question.to_uppercase().trim().starts_with("DO") |
                    question.to_uppercase().trim().starts_with("MAY")
                => {
                    true
                }
                &_ => { false }
            }
        } else {
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

        assert_eq!(
            seer.answers.iter().any(|&answer| answer.contains("Your chances are good")),
            true
        );
        assert_eq!(
            seer.answers.iter().any(|&answer| answer.contains("Come again real soon")),
            false
        );
    }

    #[test]
    fn random_answers() {
        let seer = MysticSeer::new();

        let mut rand_answer = seer.get_random_answer();
        assert_eq!(
            seer.answers.iter().any(|&answer| answer == rand_answer),
            true
        );

        rand_answer = "I am just a napkin dispenser.".to_string();
        assert_eq!(
            seer.answers.iter().any(|&answer| answer == rand_answer),
            false
        );
    }

    #[test]
    fn validation() {
        let seer = MysticSeer::new();

        let mut question = " Can I become a mystic seer?";
        assert_eq!(seer.validate_yes_no_question(question), true);

        question = "Does she love me?";
        assert_eq!(seer.validate_yes_no_question(question), true);

        question = "What's the future looking like?";
        assert_eq!(seer.validate_yes_no_question(question), false);

        question = "Do I have the illest style of them all";
        assert_eq!(seer.validate_yes_no_question(question), false);

        question = "ANSWER ME YOU DEVIL";
        assert_eq!(seer.validate_yes_no_question(question), false);
    }
}
