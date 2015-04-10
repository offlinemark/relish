use std::io;
use std::io::Write; // need it to flush stdout
use std::env;
use std::path;
use std::process;
use std::process::Stdio;
use std::process::Command;

static BUILTINS: [&'static str; 3] = ["exit", "cd", "pwd"];

struct CommandLine {
    cmd: String,
    args: Vec<String>
}


fn clear_cmdline(cmdline: &mut CommandLine) {
    cmdline.cmd.clear();
    cmdline.args.clear();
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


fn builtin(cmdline: &CommandLine) {
    match &cmdline.cmd[..] {
        "exit" => {
            println!("So long, and thanks for all the fish!");
            process::exit(0);
        }
        "cd" => {
            // these two are declared here to satisfy lifetime requirements.
            // because dir (below) is a pointer, the objects it can point to
            // must have a longer life, and be declared before
            let home = env::home_dir().unwrap_or(path::PathBuf::from("."));
            let old = env::var("OLDPWD").unwrap_or(".".to_string());

            // get dir to change to based on the length of cmdline.args
            let dir = if cmdline.args.len() == 0 {
                home.as_path()
            } else {
                if cmdline.args[0] == "-" {
                    path::Path::new(&old)
                } else {
                    path::Path::new(&cmdline.args[0])
                }
            };

            // set $OLDPWD
            env::set_var("OLDPWD", &env::current_dir().unwrap());
            // change directory
            if let Err(why) = env::set_current_dir(&dir) {
               println!("relish: {}", why);
            }
        }
        "pwd" => {
            let pwd = env::current_dir().unwrap();
            let pwd = pwd.display();
            println!("{}", pwd);
        }
        _ => {}
    }
}


/*
 * get_prompt - generate and return shell prompt
 */
fn get_prompt() -> String {
    // get username
    let username = env::var("USER").unwrap_or("???".to_string());

    // get hostname
    let hostname = Command::new("/bin/hostname").output().unwrap();
    let hostname = String::from_utf8_lossy(&hostname.stdout);
    let hostname = hostname.trim();

    // get current directory
    let pwd = env::current_dir().unwrap();
    let pwd = pwd.display();

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
        if each.trim() == "" {
            continue;
        } else if each.trim().chars().nth(0).unwrap() == '#' {
            // ok to use unwrap because we've guaranteed input isn't empty
            break;
        } else if i == 0 {
            cmdline.cmd = each.trim().to_string();
        } else {
            cmdline.args.push(each.trim().to_string());
        }
    }
}


fn main() {
    // init
    let mut cmdline: CommandLine = CommandLine {
        cmd: String::new(),
        args: Vec::new()
    };

    // main shell loop
    loop {
        // print prompt
        print!("{}", get_prompt());
        if let Err(why) = io::stdout().flush() {
            println!("relish: {}", why);
            continue;
        }

        // clear contents of last command
        if !cmdline.cmd.is_empty() {
            clear_cmdline(&mut cmdline);
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

        // check if blank/comment
        cmdline.cmd = cmdline.cmd.trim().to_string();
        match cmdline.cmd.chars().nth(0) {
            Some(first) =>
                if first == '#' {
                    continue
                },
            None => continue // empty string
        }

        // parse
        preprocess(&mut cmdline);

        // handle builtins
        if BUILTINS.contains(&&cmdline.cmd[..]) {
            builtin(&cmdline)
        } else {
            execute(&cmdline);
        }
    }
}
