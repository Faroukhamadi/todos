use clap::{arg, Command};

fn cli() -> Command {
    Command::new("tds")
        .about("A tool to manage your todos")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("list").about("lists todos"))
}

fn main() {
    //
}
