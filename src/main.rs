use std::io;
use std::io::Write; // need it to flush stdout
use std::env;
use std::process;


fn execute(cmd: &str) {
    match process::Command::new(cmd).output() {
        Ok(ret) => println!("{}", String::from_utf8_lossy(&ret.stdout).trim()),
        Err(why) => println!("rush: {}", why)
    }
}


fn get_prompt() -> String {
    let mut prompt: String = String::new();

    // get username
    prompt.push_str(env::var("USER").unwrap().trim());

    prompt.push('@');

    // get hostname
    let hn = process::Command::new("/bin/hostname").output().unwrap();
    prompt.push_str(String::from_utf8_lossy(&hn.stdout).trim());

    prompt.push(' ');

    // get current directory
    prompt.push_str(env::var("PWD").unwrap().trim());

    prompt.push_str(" $ ");
    prompt
}


fn main() {

    // allocate String for user input
    let mut input: String = String::new();
    let prompt: String = get_prompt();

    loop {

        print!("{}", prompt);
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

        // catch builtins, otherwise feed to execute function
        match input.trim() {
            "" => continue,
            "exit" => {
                println!("Exiting!");
                break;
            },
            cmd => execute(&cmd)
        }
    }
}
