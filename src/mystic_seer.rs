use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

pub struct MysticSeer<'a> {
    answers: Vec<&'a str>,
}

impl<'a> MysticSeer<'a> {
    pub fn new() -> MysticSeer<'a> {
        MysticSeer {
            answers: vec!["Why Not?"],
        }
    }
    pub fn prompt(&self) -> Result<String> {
        let mut rl = rustyline::DefaultEditor::new()?;
        let readline = rl.readline("Put in a penny? Y/N \n >> ")?;
       if readline.trim() == "Y" {
        println!("You selected yes.");
            println!("{}", self.ask_seer().unwrap());
       }
       if readline.trim() == "N"{
        println!("You selected no. Just in the nick of time.");
       }
       if readline.trim() != "N" && readline.trim() != "Y" {
        println!("What the hell are you trying to pull here?");
        self.prompt();
       }
       Ok(readline)
    }

    pub fn ask_seer(&self) -> Result<String>{
        let mut rl = rustyline::DefaultEditor::new()?;
        let readline = rl.readline("Ask the mystic seer a yes or no question.");
        //TODO: Validate answer
        //TODO: randomly return an answer from the seer
        Ok(readline?)
    }
    
}

#[cfg(test)]
mod tests {
    use super::MysticSeer;
    #[test]
    fn basics() {
        let seer = MysticSeer::new();
        assert_eq!(seer.prompt(), "Put in a penny? Y/N");
        assert_eq!(seer.answers[0], "Why Not?");
    }
}

/*
 let ans:&[&str] = &["What Do You Think?", "Try Again"];
    println!("Insert 10 cents? Y/N");
    let mut input_str = String::new();
    while input_str.trim() != "N"{
        input_str.clear();
        io::stdin().read_line(&mut input_str);
        if input_str.trim() == "Y"{
            println!("Please ask a Yes Or No question");
            io::stdin().read_line(&mut input_str);
            println!("{}", ans[0])
        }
    }
    println!("Come back real soon!")
    */
