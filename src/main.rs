use std::io;
use std::io::Write; // need it to flush stdout
use std::env;
use std::process;


struct CommandLine<'a> {
    cmd: String,
    args: Vec<&'a str>
}


fn execute(cmdline: &CommandLine) {
    match process::Command::new(&cmdline.cmd).args(&cmdline.args).output() {
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

    // init
    let mut cmdline: CommandLine = CommandLine {
        cmd: String::new(),
        args: Vec::new()
    };
    let prompt: String = get_prompt();

    // main shell loop
    loop {
        // print prompt
        print!("{}", prompt);
        if let Err(why) = io::stdout().flush() {
            println!("error: {}", why);
            continue;
        }

        // clear contents of last command
        if !cmdline.cmd.is_empty() {
            cmdline.cmd.clear();
        }

        // read input into our String. if there was an error, print the
        // error message and restart loop
        if let Err(why) = io::stdin().read_line(&mut cmdline.cmd){
            println!("error: {}", why);
            continue;
        }

        // trim whitespace
        cmdline.cmd = cmdline.cmd.trim().to_string();

        // catch builtins, otherwise feed to execute function
        match &cmdline.cmd[..] { // coerce String to &str
            "" => continue,
            "exit" => {
                println!("Exiting!");
                break;
            },
            _ => execute(&cmdline)
        }
    }
}
