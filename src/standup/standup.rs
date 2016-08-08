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

pub enum Aspect {
    Today,
    Yesterday,
    Blocker
}

impl Display for Standup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}\n", self.date.format("%F - %A")));
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

    pub fn set_date(self, date: Date<Local>) -> Standup {
        Standup { date: date, .. self }
    }

    pub fn remove(self, aspect: Aspect, index: usize) -> Standup {
        match aspect {
            Aspect::Today => self.remove_today(index),
            Aspect::Yesterday => self.remove_yesterday(index),
            Aspect::Blocker => self.remove_blocker(index)
        }
    }

    pub fn add(self, aspect: Aspect, msg: &str) -> Standup {
        match aspect {
            Aspect::Today => self.add_today(&msg),
            Aspect::Yesterday => self.add_yesterday(&msg),
            Aspect::Blocker => self.add_blocker(&msg)
        }
    }

    fn remove_blocker(self, index: usize) -> Standup {
        Standup { blocker: Standup::delete(&self.blocker, index), ..self }
    }

    fn remove_today(self, index: usize) -> Standup {
        Standup { today: Standup::delete(&self.today, index), ..self }
    }

    fn remove_yesterday(self, index: usize) -> Standup {
        Standup { yesterday: Standup::delete(&self.yesterday, index), ..self }
    }

    fn add_blocker(self, msg: &str) -> Standup {
        Standup { blocker: Standup::push(&self.blocker, &msg), .. self }
    }

    fn add_today(self, msg: &str) -> Standup {
        Standup { today: Standup::push(&self.today, &msg), .. self }
    }

    fn add_yesterday(self, msg: &str) -> Standup {
        Standup { yesterday: Standup::push(&self.yesterday, &msg), .. self }
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
        let standup = Standup::new().add(Aspect::Blocker, "yo yo");
        assert_eq!(standup.is_blocked(), true);
    }

    #[test]
    fn it_can_detect_non_blockage() {
        let standup = Standup::new();
        assert_eq!(standup.is_blocked(), false);
    }

    #[test]
    fn it_can_add_to_today() {
        let standup = Standup::new().add(Aspect::Today, "hello world");
        assert_eq!(standup.today.len(), 1);
        assert_eq!(standup.today[0], "hello world");
        assert_eq!(standup.add(Aspect::Today, "another").today.len(), 2);
    }

    #[test]
    fn it_can_add_to_yesterday() {
        let standup = Standup::new().add(Aspect::Yesterday, "hello world");
        assert_eq!(standup.yesterday.len(), 1);
        assert_eq!(standup.yesterday[0], "hello world");
        assert_eq!(standup.add(Aspect::Yesterday, "another").yesterday.len(), 2);
    }

    #[test]
    fn it_can_remove_a_blocker() {
        let standup = Standup::new()
            .add(Aspect::Blocker, "hello world")
            .add(Aspect::Blocker, "another world")
            .add(Aspect::Blocker, "a whole new world")
            .remove(Aspect::Blocker, 0);
        assert_eq!(standup.blocker.len(), 2);
    }

    #[test]
    fn it_will_just_return_a_copy_when_index_outside_range() {
        let standup = Standup::new()
            .add(Aspect::Blocker, "hello world")
            .add(Aspect::Blocker, "anohter world")
            .add(Aspect::Blocker, "a whole new world")
            .remove(Aspect::Blocker, 8);
        assert_eq!(standup.blocker.len(), 3);
    }

    #[test]
    fn it_can_remove_a_today() {
        let standup = Standup::new()
            .add(Aspect::Today, "hello world")
            .remove(Aspect::Today, 0);
        assert_eq!(standup.today.len(), 0);
    }

    #[test]
    fn it_can_remove_a_yesterday() {
        let standup = Standup::new()
            .add(Aspect::Yesterday, "hello world")
            .remove(Aspect::Yesterday, 0);
        assert_eq!(standup.yesterday.len(), 0);
    }
}
