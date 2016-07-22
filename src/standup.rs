use std::vec::Vec;
use chrono::Date;
use chrono::offset::local::Local;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Standup {
    pub today: Vec<String>,
    pub yesterday: Vec<String>,
    pub blocker: Vec<String>,
    pub date: Date<Local>,
}

impl Standup {
    pub fn new() -> Standup {
        Standup {
            today: vec![],
            yesterday: vec![],
            blocker: vec![],
            date: Local::today()
        }
    }

    pub fn from_date(date: Date<Local>) -> Standup {
        Standup {
            today: vec![],
            yesterday: vec![],
            blocker: vec![],
            date: date
        }
    }

    pub fn is_blocked(&self) -> bool {
        !self.blocker.is_empty()
    }

    pub fn add_blocker(self, blocker: &str) -> Standup {
        let blocker = Standup::push(&self.blocker, &blocker);
        Standup { blocker: blocker, .. self }
    }

    pub fn add_today(self, today: &str) -> Standup {
        let today = Standup::push(&self.today, &today);
        Standup { today: today, .. self }
    }

    pub fn add_yesterday(self, yesterday: &str) -> Standup {
        let yesterday = Standup::push(&self.yesterday, &yesterday);
        Standup { yesterday: yesterday, .. self }
    }

    pub fn set_date(self, date: Date<Local>) -> Standup {
        Standup { date: date, .. self }
    }

    fn push(old: &Vec<String>, message: &str) -> Vec<String> {
        let mut destination = Vec::with_capacity(old.len());
        destination.extend_from_slice(old.as_slice());
        destination.push(message.to_string());
        destination.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_detect_blockage() {
        let standup = Standup::new().add_blocker("yo yo");
        assert_eq!(standup.is_blocked(), true);
    }

    #[test]
    fn it_can_detect_non_blockage() {
        let standup = Standup::new();
        assert_eq!(standup.is_blocked(), false);
    }

    #[test]
    fn it_can_add_to_today() {
        let standup = Standup::new().add_today("hello world");
        assert_eq!(standup.today.len(), 1);
        assert_eq!(standup.today[0], "hello world");
        assert_eq!(standup.add_today("another").today.len(), 2);
    }

    #[test]
    fn it_can_add_to_yesterday() {
        let standup = Standup::new().add_yesterday("hello world");
        assert_eq!(standup.yesterday.len(), 1);
        assert_eq!(standup.yesterday[0], "hello world");
        assert_eq!(standup.add_yesterday("another").yesterday.len(), 2);
    }
}
