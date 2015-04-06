use std::io::Write; // need it to flush stdout

static PROMPT: &'static str = "> ";

fn main() {

    // we allocate a String for the user input
    let mut input: String = String::new();

    loop {

        print!("{}", PROMPT);
        // the print! macro line buffers and doesn't automatically flush
        std::io::stdout().flush();

        input.clear();

        // read input into our String
        std::io::stdin().read_line(&mut input);

        // trim the newline off and save it back
        input = input.trim().to_string();

        println!("you entered [{}]", input);

        if input == "exit" {
            println!("Exiting!");
            break;
        }
    }

}

