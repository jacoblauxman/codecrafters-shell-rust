use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let env_path = std::env::var("PATH").unwrap();

    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();

        let _ = stdin.read_line(&mut input).unwrap();
        let cmd = input.trim();

        if cmd.is_empty() {
            continue;
        }

        match parse_command(&input) {
            Some(ShellCommand::Exit(n)) => std::process::exit(n),
            Some(ShellCommand::Echo(echo)) => print!("{echo}\n"),
            Some(ShellCommand::Type(cmd)) => get_command_type(cmd, &env_path),
            Some(ShellCommand::Pwd) => {
                print!("{}\n", std::env::current_dir().unwrap().to_str().unwrap())
            }
            Some(ShellCommand::Cd(path)) => {
                if path == "~" {
                    let home_path = std::env::var("HOME").expect("Failed get `HOME` env key");

                    std::env::set_current_dir(home_path)
                        .expect("Failed to set path to environment var `HOME`'s directory");
                } else {
                    if std::env::set_current_dir(path).is_err() {
                        print!("cd: {}: No such file or directory\n", path);
                    }
                }
            }
            Some(ShellCommand::Program((cmd, args))) => {
                if let Some(ShellCommand::Program((cmd, path))) = is_executable(cmd, &env_path) {
                    let full_path = format!("{}/{}", path, cmd);
                    let args = args.split_whitespace().collect::<Vec<&str>>();

                    let output = std::process::Command::new(&full_path)
                        .args(args)
                        .output()
                        .expect("Failed to run executable program");

                    print!("{}", String::from_utf8_lossy(&output.stdout));
                } else {
                    print!("{}: command not found\n", cmd.trim());
                }
            }
            _ => print!("{}: command not found\n", input.trim()),
        };
    }
}

fn parse_command(input: &str) -> Option<ShellCommand> {
    let (cmd, args) = match input.split_once(' ') {
        Some((cmd, args)) => (cmd, args),
        None => (input.trim(), ""),
    };

    match cmd {
        "exit" => Some(ShellCommand::Exit(args.trim().parse::<i32>().unwrap_or(0))),
        "echo" => Some(ShellCommand::Echo(args.trim())),
        "type" => Some(ShellCommand::Type(args.trim())),
        "pwd" => Some(ShellCommand::Pwd),
        "cd" => Some(ShellCommand::Cd(args.trim())),
        _ => Some(ShellCommand::Program((cmd, args.trim()))),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ShellCommand<'a> {
    Exit(i32),
    Echo(&'a str),
    Type(&'a str),
    Program((&'a str, &'a str)),
    Pwd,
    Cd(&'a str),
}

#[derive(Debug, Clone)]
pub enum CommandType<'a> {
    BuiltIn,
    Executable((&'a str, &'a str)),
    Unrecognized(String),
}

fn get_command_type<'a>(cmd: &'a str, env_path: &'a str) {
    let cmd_type = match parse_command(cmd) {
        Some(ShellCommand::Echo(_)) => CommandType::BuiltIn,
        Some(ShellCommand::Exit(_)) => CommandType::BuiltIn,
        Some(ShellCommand::Type(_)) => CommandType::BuiltIn,
        Some(ShellCommand::Pwd) => CommandType::BuiltIn,
        Some(ShellCommand::Cd(_)) => CommandType::BuiltIn,
        Some(ShellCommand::Program((cmd, _))) => {
            if let Some(ShellCommand::Program((cmd, full_path))) = is_executable(cmd, env_path) {
                CommandType::Executable((cmd, full_path))
            } else {
                CommandType::Unrecognized(format!("{cmd}: not found\n"))
            }
        }
        _ => CommandType::Unrecognized(format!("{cmd} not found\n")),
    };

    match cmd_type {
        CommandType::BuiltIn => print!("{} is a shell builtin\n", cmd),
        CommandType::Executable((cmd, path)) => {
            print!("{} is {}/{}\n", cmd, path, cmd);
        }
        CommandType::Unrecognized(msg) => print!("{}", msg),
    };
}

fn is_executable<'a>(cmd: &'a str, env_path: &'a str) -> Option<ShellCommand<'a>> {
    let mut exe_paths = env_path.split(":");

    if let Some(path) = exe_paths.find(|path| std::fs::metadata(format!("{path}/{cmd}")).is_ok()) {
        return Some(ShellCommand::Program((cmd, path)));
    } else {
        None
    }
}
