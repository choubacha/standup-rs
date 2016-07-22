extern crate clap;
extern crate chrono;
extern crate serde;
extern crate serde_json;

use chrono::*;
use std::env::home_dir;
use std::fs::OpenOptions;
use clap::{Arg, App, SubCommand, ArgMatches};

mod standup;
mod manager;
mod jsonify;

use manager::Manager;
use standup::Standup;

static TYPES: &'static [&'static str] = &["today", "yesterday", "blocker"];

fn main() {
    let date_arg = Arg::with_name("date")
        .short("d")
        .long("date")
        .value_name("DATE")
        .help("The date that the standup happens on");
    let message_arg = Arg::with_name("message")
        .value_name("MESSAGE")
        .required(true)
        .help("The message to add to the stand up");

    let matches = App::new("standup")
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
                        .arg(message_arg.clone())
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
        ("today",       Some(sub_args)) => handle_today(sub_args),
        ("yesterday",   Some(sub_args)) => handle_yesterday(sub_args),
        ("blocker",     Some(sub_args)) => handle_blocker(sub_args),
        ("show",        Some(sub_args)) => handle_show(sub_args),
        ("delete",      Some(sub_args)) => handle_delete(sub_args),
        _ => {},
    }
}

fn get_date(args: &ArgMatches) -> Date<Local> {
    match args.value_of("date") {
        Some(date_string) => {
            let date = NaiveDate::parse_from_str(date_string, "%F").unwrap();
            Local.ymd(date.year(), date.month(), date.day())
        },
        _ => Local::today()
    }
}

fn load_manager() -> Manager {
    let mut path = home_dir().unwrap();
    path.push(".standup.json");
    if path.as_path().is_file() {
        let file = OpenOptions::new().read(true).open(path.as_path());
        Manager::from_reader(file.unwrap())
    } else {
        OpenOptions::new().create(true).write(true).open(path.as_path());
        Manager::new()
    }
}

fn flush_manager(manager: &mut Manager) {
    let mut path = home_dir().unwrap();
    path.push(".standup.json");
    let file = OpenOptions::new().write(true).open(path.as_path());
    manager.flush(file.unwrap());
}

fn handle_today(args: &ArgMatches) {
    let today = args.value_of("message").unwrap();
    let mut manager = load_manager();
    let date = get_date(&args);
    let standup = manager.get(&date).unwrap_or(Standup::from_date(date));
    let standup = standup.add_today(today);
    manager.insert(&standup);
    flush_manager(&mut manager)
}

fn handle_yesterday(args: &ArgMatches) {
    println!("yesterday!");
    println!("date: {}", args.value_of("date").unwrap_or(""));
}

fn handle_blocker(args: &ArgMatches) {
    println!("blocker!");
    println!("date: {}", args.value_of("date").unwrap_or(""));
}

fn handle_show(args: &ArgMatches) {
    println!("showing!");
    println!("date: {}", args.value_of("date").unwrap_or(""));
}

fn handle_delete(args: &ArgMatches) {
    println!("deleting!");
    println!("date: {}", args.value_of("date").unwrap_or(""));
}
