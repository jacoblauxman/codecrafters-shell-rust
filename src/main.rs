use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
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
            Some(ShellCommand::Type(cmd)) => command_type(cmd),
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
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub enum ShellCommand<'a> {
    Exit(i32), // process::exit expects an i32 value
    Echo(&'a str),
    Type(&'a str),
}

#[derive(Debug, Clone)]
pub enum CommandType {
    BuiltIn,
    Unrecognized,
}

fn command_type(cmd: &str) {
    let cmd_type = match parse_command(cmd) {
        Some(ShellCommand::Echo(_)) => CommandType::BuiltIn,
        Some(ShellCommand::Exit(_)) => CommandType::BuiltIn,
        Some(ShellCommand::Type(_)) => CommandType::BuiltIn,
        _ => CommandType::Unrecognized,
    };

    match cmd_type {
        CommandType::BuiltIn => print!("{} is a shell builtin\n", cmd),
        CommandType::Unrecognized => print!("{} not found\n", cmd),
    };
}
