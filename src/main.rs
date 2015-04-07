use std::io;
use std::io::Write; // need it to flush stdout
use std::process;

static PROMPT: &'static str = "$ ";

fn execute(cmd: &String) {
    let ret = process::Command::new(cmd).output().unwrap();
    println!("{}", String::from_utf8_lossy(&ret.stdout).trim());
}

fn main() {

    // we allocate a String for the user input
    let mut input: String = String::new();

    loop {

        print!("{}", PROMPT);
        if let Err(why) = io::stdout().flush() {
            println!("error: {}", why);
            continue;
        }

        // input probably has stuff in it from the last command, so clear
        // it out
        input.clear();

        // read input into our String. if there was an error, print the
        // error message and continue
        if let Err(why) = io::stdin().read_line(&mut input){
            println!("error: {}", why);
            continue;
        }

        // trim the newline off and save it back
        input = input.trim().to_string();

        execute(&input);

        if input == "exit" {
            println!("Exiting!");
            break;
        }
    }

}

