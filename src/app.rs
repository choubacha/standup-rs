use std::path::PathBuf;
use std::env::home_dir;
use std::fs::OpenOptions;
use chrono::*;
use ::CliError;
use ::StandupError;
use standup::{Aspect, Standup, Manager};

pub struct App {
    date: Date<Local>,
    manager: Manager,
}

///
/// ## App
///
/// The App is how the CLI will interact with the data. It will manager loading
/// all the data from the file and determining the current day. This should have
/// a mapping to the different commands that the client would interact with.
///
impl App {
    /// Creates a new App.
    ///
    /// Takes an optional string for the date.
    ///
    /// Returns the new App if nothing errors when loading up the data.
    pub fn new(date: Option<String>) -> Result<App, CliError> {
        let manager = try!(App::load_manager());
        let date = try!(App::get_date(date));
        Ok(App { manager: manager, date: date })
    }

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

    fn get_date(date: Option<String>) -> Result<Date<Local>, CliError> {
        match date {
            Some(date_string) => {
                NaiveDate::parse_from_str(&date_string, "%F")
                    .or(Err(CliError::Cli(StandupError::InvalidDate)))
                    .map(|date| Local.ymd(date.year(), date.month(), date.day()))
            },
            _ => Ok(Local::today())
        }
    }


    fn load_manager() -> Result<Manager, CliError> {
        App::get_path().and_then(|path| {
            if path.is_file() {
                let file = OpenOptions::new().read(true).open(path);
                file.map_err(CliError::Io).and_then(Manager::from_reader)
            } else {
                OpenOptions::new().create(true).write(true).open(path);
                Ok(Manager::new())
            }
        })
    }

    fn flush_manager(&mut self) -> Result<(), CliError> {
        App::get_path().and_then(|path| {
            let file = OpenOptions::new().create(true).write(true).truncate(true).open(path);
            file.map_err(CliError::Io).and_then(|file| self.manager.flush(file))
        })
    }

    pub fn get_standup(&self) -> Standup {
        self.manager.get(&self.date).unwrap_or(Standup::from_date(self.date.clone()))
    }

    pub fn standups(&self) -> Vec<&Standup> {
        self.manager.standups()
    }

    pub fn record(&mut self, aspect: Aspect, message: String) {
        let standup = self.get_standup().add(aspect, &message);
        self.manager.insert(standup);
        self.flush_manager();
    }

    pub fn delete(&mut self) -> Option<Standup> {
        let standup = self.manager.delete(&self.date);
        self.flush_manager();
        standup
    }
}
