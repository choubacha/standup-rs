use standup::Standup;
use std::io::{Read,Write};
use chrono::Date;
use chrono::offset::local::Local;
use std::collections::BTreeMap;
use jsonify;
use ::CliError;

pub struct Manager {
    standups: BTreeMap<Date<Local>, Standup>
}

impl Manager {
    pub fn new() -> Manager {
        Manager { standups: BTreeMap::new() }
    }

    pub fn from_reader<F: Read>(mut reader: F) -> Result<Manager, CliError> {
        let mut buf = String::new();
        try!(reader.read_to_string(&mut buf).map_err(CliError::Io));

        let mut manager = Manager { standups: BTreeMap::new() };
        for standup in jsonify::deserialize(buf).unwrap() {
            manager.insert(standup);
        }
        Ok(manager)
    }

    pub fn flush<F: Write>(&self, mut writer: F) -> Result<(), CliError> {
        let standups: Vec<&Standup> = self.standups.values().clone().collect();
        writer.write(jsonify::serialize(&standups).as_bytes())
            .map(|_| ())
            .map_err(CliError::Io)
    }

    pub fn standups(&self) -> Vec<&Standup> {
        self.standups.values().collect()
    }

    pub fn get(&self, date: &Date<Local>) -> Option<Standup> {
        self.standups.get(&date).map(|standup| standup.clone())
    }

    pub fn insert(&mut self, standup: Standup) {
        self.standups.insert(standup.date.clone(), standup);
    }

    pub fn delete(&mut self, date: &Date<Local>) -> Option<Standup> {
        self.standups.remove(date)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::*;
    use standup::Standup;
    use std::str;

    #[test]
    fn it_can_read_the_standups_out_of_a_stream() {
        let manager = Manager::from_reader("[{\"date\":\"2015-01-01\"}]".as_bytes()).unwrap();
        assert_eq!(manager.get(&Local.ymd(2015,1,1)).is_some(), true);
    }

    #[test]
    fn it_can_flush_to_a_stream() {
        let mut manager = Manager::from_reader("[]".as_bytes()).unwrap();
        manager.insert(Standup::from_date(Local.ymd(2015, 1, 1)));
        let mut bytes: Vec<u8> = Vec::new();
        manager.flush(&mut bytes).unwrap();
        let json = str::from_utf8(bytes.as_slice()).unwrap();
        assert_eq!(json.contains("\"date\":\"2015-01-01\""), true);
    }

    #[test]
    fn it_can_add_standups() {
        let mut manager = Manager::from_reader("[]".as_bytes()).unwrap();
        let standup = Standup::new();
        manager.insert(standup.clone());
        assert_eq!(manager.get(&standup.date).unwrap(), standup);
    }

    #[test]
    fn it_can_delete_a_standup() {
        let mut manager = Manager::from_reader("[]".as_bytes()).unwrap();
        manager.insert(Standup::from_date(Local.ymd(2015, 1, 1)));
        assert_eq!(manager.standups.len(), 1);
        manager.delete(&Local.ymd(2015, 1, 1));
        assert_eq!(manager.standups.len(), 0);
    }
}

