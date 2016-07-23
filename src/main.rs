extern crate clap;
extern crate chrono;
extern crate serde;
extern crate serde_json;

use chrono::*;
use std::env::home_dir;
use std::fs::OpenOptions;
use std::io;
use clap::{Arg, App, SubCommand, ArgMatches};

mod standup;
mod manager;
mod jsonify;

use manager::Manager;
use standup::Standup;

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
    DataFilepathInvalid
}

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
        ("today",       Some(sub_args)) => handle_today(sub_args),
        ("yesterday",   Some(sub_args)) => handle_yesterday(sub_args),
        ("blocker",     Some(sub_args)) => handle_blocker(sub_args),
        ("show",        Some(sub_args)) => handle_show(sub_args),
        ("list",        Some(sub_args)) => handle_list(sub_args),
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

use std::path::PathBuf;
fn get_path() -> Result<PathBuf, CliError> {
    match home_dir() {
        Some(ref mut path_buf) => {
            path_buf.push(".standup.json");
            // Recreate as immutable
            Ok(path_buf.as_path().to_path_buf())
        },
        None => Err(CliError::Cli(StandupError::HomeDirNotFound))
    }
}

fn load_manager() -> Result<Manager, CliError> {
    get_path().and_then(|path| {
        if path.is_file() {
            let file = OpenOptions::new().read(true).open(path);
            file.map_err(CliError::Io).and_then(Manager::from_reader)
        } else {
            OpenOptions::new().create(true).write(true).open(path);
            Ok(Manager::new())
        }
    })
}

fn flush_manager(manager: &mut Manager) -> Result<(), CliError> {
    get_path().and_then(|path| {
        let file = OpenOptions::new().write(true).open(path);
        file.map_err(CliError::Io).and_then(|file| manager.flush(file))
    })
}

fn with_standup<F>(args: &ArgMatches, op: F) -> Result<(), CliError>
    where F: FnOnce(Standup) -> Standup
{
    let date = get_date(&args);
    with_manager(|manager| {
        let standup = op(manager.get(&date).unwrap_or(Standup::from_date(date)));
        Some(standup)
    })
}

fn with_ro_standup<F>(args: &ArgMatches, op: F) -> Result<(), CliError>
    where F: FnOnce(Standup)
{
    let date = get_date(&args);
    with_manager(|manager| {
        op(manager.get(&date).unwrap_or(Standup::from_date(date)));
        None
    })
}

fn with_manager<F>(op: F) -> Result<(), CliError>
    where F: FnOnce(&mut Manager) -> Option<Standup>
{
    let mut manager = try!(load_manager());
    if let Some(standup) = op(&mut manager) {
        manager.insert(&standup);
        flush_manager(&mut manager)
    } else {
        Ok(())
    }
}

fn handle_today(args: &ArgMatches) {
    let today = args.value_of("message").unwrap();
    with_standup(&args, |standup| standup.add_today(today)).unwrap();
}

fn handle_yesterday(args: &ArgMatches) {
    let yesterday = args.value_of("message").unwrap();
    with_standup(&args, |standup| standup.add_yesterday(yesterday)).unwrap();
}

fn handle_blocker(args: &ArgMatches) {
    let blocker = args.value_of("message").unwrap();
    with_standup(&args, |standup| standup.add_blocker(blocker)).unwrap();
}

fn handle_show(args: &ArgMatches) {
    with_ro_standup(&args, |standup| println!("{}", &standup));
}

fn handle_list(args: &ArgMatches) {
    let mut manager = load_manager().unwrap();
    for standup in manager.standups() {
        println!("{}", &standup);
    }
}

fn handle_delete(args: &ArgMatches) {
    println!("deleting!");
    println!("date: {}", args.value_of("date").unwrap_or(""));
}
