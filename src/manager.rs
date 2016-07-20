use standup::Standup;
use std::path::Path;

pub struct Manager {
    path: Box<Path>,
    standups: Vec<Standup>
}

impl Manager {
    // pub fn new(path: Path) -> Manager {
    //     Manager { path: path, standups: standups }
    // }

    // pub fn write(&mut self, standup: &Standup) -> Result<()> {
    //     Err(())
    // }

    // pub fn get(&mut self) -> Result<Standup, ()> {
    //     Err(())
    // }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Date;
    use chrono::offset::local::Local;

    // #[test]
    // fn it_can_upsert_a_record() {
    //     let standup = Standup::from_date(Local::ymd(2016, 7, 15));
    //     Manager::new("/tmp/standup.yml").write(&standup);
    // }

    // #[test]
    // fn it_can_retrieve_a_record_by_date() {
    // }
}

