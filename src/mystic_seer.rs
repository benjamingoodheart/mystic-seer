use std::io;

struct MysticSeer{
    answers: Vec<String>,
}

impl MysticSeer{
    pub fn prompt() -> String{
        let prompt_str = "Put in a penny?";
        prompt_str.to_string()
    }
}


#[cfg(test)]
    mod tests{
        use super::MysticSeer;
        #[test]
        fn basics(){

            assert_eq!(MysticSeer::prompt(),"Put in a penny?");
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
