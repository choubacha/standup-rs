use std::vec::Vec;
use std::fmt;
use std::fmt::Display;
use chrono::Date;
use chrono::offset::local::Local;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Standup {
    pub today: Vec<String>,
    pub yesterday: Vec<String>,
    pub blocker: Vec<String>,
    pub date: Date<Local>,
}

impl Display for Standup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "date: {}\n", self.date.format("%F - %A")));
        try!(write!(f, "  today:\n"));
        for (i, message) in self.today.iter().enumerate() {
            try!(write!(f, "    {}. {}\n", i + 1, message));
        }
        try!(write!(f, "  yesterday:\n"));
        for (i, message) in self.yesterday.iter().enumerate() {
            try!(write!(f, "    {}. {}\n", i + 1, message));
        }
        if self.is_blocked() {
            try!(write!(f, "  blocker:\n"));
            for (i, message) in self.blocker.iter().enumerate() {
                try!(write!(f, "    {}. {}\n", i + 1, message));
            }
        }
        Ok(())
    }
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

    pub fn remove_blocker(self, index: usize) -> Standup {
        Standup { blocker: Standup::delete(&self.blocker, index), ..self }
    }

    pub fn remove_today(self, index: usize) -> Standup {
        Standup { today: Standup::delete(&self.today, index), ..self }
    }

    pub fn remove_yesterday(self, index: usize) -> Standup {
        Standup { yesterday: Standup::delete(&self.yesterday, index), ..self }
    }

    pub fn add_blocker(self, msg: &str) -> Standup {
        Standup { blocker: Standup::push(&self.blocker, &msg), .. self }
    }

    pub fn add_today(self, msg: &str) -> Standup {
        Standup { today: Standup::push(&self.today, &msg), .. self }
    }

    pub fn add_yesterday(self, msg: &str) -> Standup {
        Standup { yesterday: Standup::push(&self.yesterday, &msg), .. self }
    }

    pub fn set_date(self, date: Date<Local>) -> Standup {
        Standup { date: date, .. self }
    }

    fn delete(old: &Vec<String>, index: usize) -> Vec<String> {
        if index >= old.len() { return old.clone() }
        let mut destination = Vec::with_capacity(old.len());
        destination.extend_from_slice(&old[..index]);
        destination.extend_from_slice(&old[(index + 1)..]);
        destination.shrink_to_fit();
        destination
    }

    fn push(old: &Vec<String>, message: &str) -> Vec<String> {
        let mut destination = Vec::with_capacity(old.len());
        destination.extend_from_slice(old.as_slice());
        destination.push(message.to_string());
        destination
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

    #[test]
    fn it_can_remove_a_blocker() {
        let standup = Standup::new()
            .add_blocker("hello world")
            .add_blocker("another world")
            .add_blocker("a whole new world")
            .remove_blocker(0);
        assert_eq!(standup.blocker.len(), 2);
    }

    #[test]
    fn it_will_just_return_a_copy_when_index_outside_range() {
        let standup = Standup::new()
            .add_blocker("hello world")
            .add_blocker("anohter world")
            .add_blocker("a whole new world")
            .remove_blocker(8);
        assert_eq!(standup.blocker.len(), 3);
    }

    #[test]
    fn it_can_remove_a_today() {
        let standup = Standup::new()
            .add_today("hello world")
            .remove_today(0);
        assert_eq!(standup.today.len(), 0);
    }

    #[test]
    fn it_can_remove_a_yesterday() {
        let standup = Standup::new()
            .add_yesterday("hello world")
            .remove_yesterday(0);
        assert_eq!(standup.yesterday.len(), 0);
    }
}
