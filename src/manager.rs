use standup::Standup;
use std::io::{Read,Write};
use chrono::Date;
use chrono::offset::local::Local;
use std::collections::BTreeMap;
use jsonify;

pub struct Manager {
    standups: BTreeMap<Date<Local>, Standup>
}

impl Manager {
    pub fn new() -> Manager {
        Manager { standups: BTreeMap::new() }
    }

    pub fn from_reader<F: Read>(mut reader: F) -> Manager {
        let mut buf = String::new();
        reader.read_to_string(&mut buf).unwrap();
        let mut manager = Manager { standups: BTreeMap::new() };
        for standup in jsonify::deserialize(buf).unwrap() {
            manager.insert(&standup);
        }
        manager
    }

    pub fn flush<F: Write>(&mut self, mut writer: F) {
        let standups: Vec<&Standup> = self.standups.values().clone().collect();
        writer.write(jsonify::serialize(&standups).as_bytes()).unwrap();
    }

    pub fn get(&mut self, date: &Date<Local>) -> Option<Standup> {
        self.standups.get(&date).map(|standup| standup.clone())
    }

    pub fn insert(&mut self, standup: &Standup) {
        self.standups.insert(standup.date.clone(), standup.clone());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::*;
    use standup::Standup;
    use std::io::Write;
    use std::str;

    #[test]
    fn it_can_read_the_standups_out_of_a_stream() {
        let mut manager = Manager::from_reader("[{\"date\":\"2015-01-01\"}]".as_bytes());
        assert_eq!(manager.get(&Local.ymd(2015,1,1)).is_some(), true);
    }

    #[test]
    fn it_can_flush_to_a_stream() {
        let mut manager = Manager::from_reader("[]".as_bytes());
        manager.insert(&Standup::from_date(Local.ymd(2015, 1, 1)));
        let mut bytes: Vec<u8> = Vec::new();
        manager.flush(&mut bytes);
        let json = str::from_utf8(bytes.as_slice()).unwrap();
        assert_eq!(json.contains("\"date\":\"2015-01-01\""), true);
    }

    #[test]
    fn it_can_add_standups() {
        let mut manager = Manager::from_reader("[]".as_bytes());
        let standup = Standup::new();
        manager.insert(&standup);
        assert_eq!(manager.get(&standup.date).unwrap(), standup);
    }
}

