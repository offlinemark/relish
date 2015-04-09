use std::io;
use std::io::Write; // need it to flush stdout
use std::env;
use std::process::Stdio;
use std::process::Command;


struct CommandLine {
    cmd: String,
    args: Vec<String>
}


/*
 * execute - execute shell command line based on input CommandLine
 */
fn execute(cmdline: &CommandLine) {
    if let Err(why) = Command::new(&cmdline.cmd)
                              .args(&cmdline.args)
                              .stdout(Stdio::inherit())
                              .stderr(Stdio::inherit())
                              .output() {
        println!("relish: {}", why);
    }
}


/*
 * get_prompt - generate and return shell prompt
 */
fn get_prompt() -> String {
    // get username
    let username = env::var("USER").unwrap();

    // get hostname
    let hostname = Command::new("/bin/hostname").output().unwrap();
    let hostname = String::from_utf8_lossy(&hostname.stdout);
    let hostname = hostname.trim();

    // get current directory
    let pwd = env::current_dir().unwrap();
    let pwd = pwd.as_path().to_str().unwrap();

    format!("{}@{} {} $ ", username, hostname, pwd)
}


/*
 * preprocess - main parsing routine responsible for popularing CommandLine
 * struct
 */
fn preprocess(cmdline: &mut CommandLine) {
    let tmp = cmdline.cmd.clone();
    // TODO: this is awful, refactor to not use a loop
    for (i, each) in tmp.split(' ').enumerate() {
        if i == 0 {
            cmdline.cmd = each.to_string();
        } else {
            cmdline.args.push(each.to_string());
        }
    }
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
            println!("relish: {}", why);
            continue;
        }

        // clear contents of last command
        if !cmdline.cmd.is_empty() {
            cmdline.cmd.clear();
            cmdline.args.clear();
        }

        // read input into our String. if bytes_read is 0, we've hit EOF
        // and should exit. if there was an error, print the
        // error message and restart loop
        match io::stdin().read_line(&mut cmdline.cmd) {
            Ok(bytes_read) =>
                // Exit on EOF (Ctrl-d, end of script)
                if bytes_read == 0 {
                    println!("");
                    break;
                },
            Err(why) => {
                println!("relish: {}", why);
                continue;
            }
        }

        // check if blank
        cmdline.cmd = cmdline.cmd.trim().to_string();
        if cmdline.cmd == "" {
            continue;
        }

        // parse
        preprocess(&mut cmdline);

        // handle builtins
        match &cmdline.cmd[..] { // coerce String to &str
            "exit" => {
                println!("Exiting!");
                break;
            }
            _ => {}
        }

        execute(&cmdline);
    }
}
