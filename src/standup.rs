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
        let mut new_blocked = self.blocked.clone();
        new_blocked.push(blocker.to_string());
        Standup { blocked: new_blocked.clone(), .. self }
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
}
