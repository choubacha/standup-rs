extern crate clap;
extern crate chrono;
extern crate serde;
extern crate serde_json;

use clap::{Arg, SubCommand, ArgMatches};
use std::io;

mod standup;
mod jsonify;
mod app;

use standup::Aspect;
use app::App;

static TYPES: &'static [&'static str] = &["today", "yesterday", "blocker"];

#[derive(Debug)]
pub enum CliError {
    Io(io::Error),
    Parse(serde_json::error::Error),
    Cli(StandupError)
}

#[derive(Debug)]
pub enum StandupError {
    HomeDirNotFound,
    DataFilepathInvalid,
    InvalidDate,
}

fn main() {
    let date_arg = Arg::with_name("date")
        .short("d")
        .long("date")
        .value_name("DATE")
        .use_delimiter(false)
        .help("The date that the standup happens on");
    let message_arg = Arg::with_name("message")
        .value_name("MESSAGE")
        .required(true)
        .use_delimiter(false)
        .help("The message to add to the stand up");

    let matches = clap::App::new("standup")
        .version("0.0.1")
        .author("Kevin Bacha <chewbacha@gmail.com>")
        .about("Manages stand up entries and keeps log")
        .subcommand(SubCommand::with_name("today")
                        .about("Manages what you will be working on")
                        .alias("t")
                        .arg(date_arg.clone())
                        .arg(message_arg.clone())
                        )
        .subcommand(SubCommand::with_name("yesterday")
                        .about("Manages what you worked on the day before")
                        .alias("y")
                        .arg(date_arg.clone())
                        .arg(message_arg.clone())
                        )
        .subcommand(SubCommand::with_name("blocker")
                        .about("Manages what is blocking you")
                        .alias("b")
                        .arg(date_arg.clone())
                        .arg(message_arg.clone())
                        )
        .subcommand(SubCommand::with_name("show")
                        .about("Displays the notes from stand up")
                        .alias("s")
                        .arg(date_arg.clone())
                        )
        .subcommand(SubCommand::with_name("list")
                        .about("Displays the notes from the last few standups")
                        .alias("ls")
                        )
        .subcommand(SubCommand::with_name("delete")
                        .about("Deletes the standup on the specified day.")
                        .alias("d")
                        .arg(date_arg.clone().required(true))
                        .arg(Arg::with_name("type")
                                 .value_name("TYPE")
                                 .requires("line_number")
                                 .possible_values(&TYPES)
                                 .index(1)
                                 .help("The type of line to delete."))
                        .arg(Arg::with_name("line_number")
                                 .value_name("LINE_NUMBER")
                                 .requires("type")
                                 .index(2)
                                 .help("The line number to delete."))
                        )
        .get_matches();

    match matches.subcommand() {
        ("today",       Some(sub_args)) => record_message(Aspect::Today, sub_args),
        ("yesterday",   Some(sub_args)) => record_message(Aspect::Yesterday, sub_args),
        ("blocker",     Some(sub_args)) => record_message(Aspect::Blocker, sub_args),
        ("show",        Some(sub_args)) => handle_show(sub_args),
        ("list",        Some(_sub_args)) => handle_list(),
        ("delete",      Some(sub_args)) => handle_delete(sub_args),
        _ => {},
    }
}

fn record_message(aspect: Aspect, args: &ArgMatches) {
    let message = args.value_of("message").map(|s| s.to_string()).unwrap();
    let date = args.value_of("date").map(|s| s.to_string());
    let mut app = App::new(date).unwrap();
    app.record(aspect, message);
}

fn handle_show(args: &ArgMatches) {
    let date = args.value_of("date").map(|s| s.to_string());
    let app = App::new(date).unwrap();
    println!("{}", &app.get_standup());
}

fn handle_list() {
    let app = App::new(None).unwrap();
    for standup in app.standups().iter().rev() {
        println!("{}", &standup);
    }
}

fn handle_delete(args: &ArgMatches) {
    let date = args.value_of("date").map(|s| s.to_string());
    let mut app = App::new(date).unwrap();
    if let Some(line_number) = args.value_of("line_number") {
        if let Ok(index) = line_number.parse::<usize>() {
            match args.value_of("type") {
                Some("today")       => app.delete_line(Aspect::Today,       index - 1),
                Some("yesterday")   => app.delete_line(Aspect::Yesterday,   index - 1),
                Some("blocker")     => app.delete_line(Aspect::Blocker,     index - 1),
                _                   => println!("Invalid aspect")
            }
            println!("{}", &app.get_standup());
        } else {
            println!("Invalid line number");
        }
    } else {
        if let Some(standup) = app.delete() {
            println!("deleted: \n{}", standup);
        } else {
            println!("No standup found on that day");
        }
    }
}
