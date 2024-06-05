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
            Some(ShellCommand::Program((cmd, args))) => {
                let cmd = is_executable(cmd, &env_path);

                match cmd {
                    Some(_) => {
                        let _ = std::process::Command::new(&env_path)
                            .args(args.split(' '))
                            .output()
                            .expect("Failed to run executable program");
                    }
                    None => {
                        print!("{}: command not found", &input)
                    }
                }
            }
            _ => print!("{}: command not found\n", input.trim()),
        };
    }
}

fn parse_command(input: &str) -> Option<ShellCommand> {
    let (cmd, args) = match input.split_once(' ') {
        Some((cmd, args)) => (cmd, args),
        None => (input, ""),
    };

    match cmd {
        "exit" => Some(ShellCommand::Exit(args.trim().parse::<i32>().unwrap_or(0))),
        "echo" => Some(ShellCommand::Echo(args.trim())),
        "type" => Some(ShellCommand::Type(args.trim())),
        _ => Some(ShellCommand::Program((cmd, args.trim()))),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ShellCommand<'a> {
    Exit(i32), // process::exit expects an i32 value
    Echo(&'a str),
    Type(&'a str),
    Program((&'a str, &'a str)),
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
