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
    // get username
    let username = env::var("USER").unwrap();

    // get hostname
    let hostname = process::Command::new("/bin/hostname").output().unwrap();
    let hostname = String::from_utf8_lossy(&hostname.stdout);
    let hostname = hostname.trim();

    // get current directory
    let pwd = env::current_dir().unwrap();
    let pwd = pwd.as_path().to_str().unwrap();

    format!("{}@{} {} $ ", username, hostname, pwd)
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
