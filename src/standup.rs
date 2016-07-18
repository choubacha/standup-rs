use std::vec::Vec;
use chrono::Date;
use chrono::offset::local::Local;

pub struct Standup {
    today: Vec<String>,
    yesterday: Vec<String>,
    blocked: Vec<String>,
    date: Date<Local>,
}

impl Standup {
    pub fn new() -> Standup {
        Standup {
            today: vec![],
            yesterday: vec![],
            blocked: vec![],
            date: Local::today()
        }
    }

    pub fn is_blocked(&self) -> bool {
        !self.blocked.is_empty()
    }

    pub fn blocker(self, blocker: &str) -> Standup {
        let blocked = Standup::push(&self.blocked, &blocker);
        Standup { blocked: blocked, .. self }
    }

    pub fn today(self, today: &str) -> Standup {
        let today = Standup::push(&self.today, &today);
        Standup { today: today, .. self }
    }

    pub fn yesterday(self, yesterday: &str) -> Standup {
        let yesterday = Standup::push(&self.yesterday, &yesterday);
        Standup { yesterday: yesterday, .. self }
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
        let standup = Standup::new().blocker("yo yo");
        assert_eq!(standup.is_blocked(), true);
    }

    #[test]
    fn it_can_detect_non_blockage() {
        let standup = Standup::new();
        assert_eq!(standup.is_blocked(), false);
    }

    #[test]
    fn it_can_add_to_today() {
        let standup = Standup::new().today("hello world");
        assert_eq!(standup.today.len(), 1);
        assert_eq!(standup.today[0], "hello world");
        assert_eq!(standup.today("another").today.len(), 2);
    }

    #[test]
    fn it_can_add_to_yesterday() {
        let standup = Standup::new().yesterday("hello world");
        assert_eq!(standup.yesterday.len(), 1);
        assert_eq!(standup.yesterday[0], "hello world");
        assert_eq!(standup.yesterday("another").yesterday.len(), 2);
    }
}
