use std::io;

struct MysticSeer<'a> {
    answers: Vec<&'a str>,
}

impl<'a> MysticSeer<'a> {
    fn new() -> MysticSeer<'a> {
        MysticSeer {
            answers: vec!["Why Not?"],
        }
    }
    pub fn prompt(&self) -> String {
        let prompt_str = "Put in a penny? Y/N";
        prompt_str.to_string()
    }

    pub fn read_input(&self) -> String {
        let mut input_str = String::new();
        //while input_str.trim() != "N" {
            //DO SOMETHING
        //}
        let msg = "See Ya Real Soon";
        msg.to_string()
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
