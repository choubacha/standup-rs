use standup::Standup;
use chrono::Date;
use serde_json::builder::{ObjectBuilder, ArrayBuilder};
use serde_json::value::Value::Array;
use serde_json::value::Value;
use std::sync::Arc;

pub fn serialize(standups: &[&Standup]) -> String {
    let array = standups.iter().map(|standup| {
        ObjectBuilder::new()
            .insert("date", standup.date.format("%F").to_string())
            .insert("today", standup.today.clone())
            .insert("yesterday", standup.yesterday.clone())
            .insert("blocked", standup.blocked.clone())
            .unwrap()
    });
    format!("{}", Array(array.collect::<Vec<Value>>()))
}

#[cfg(test)]
mod test {
    use super::*;
    use standup::Standup;
    use chrono::*;

    #[test]
    fn it_will_include_todays_notes() {
        let standup = Standup::new().add_today("today");
        let json = serialize(&[&standup]);
        assert!(json.as_str().contains("today\":[\"today\"]"));
    }

    #[test]
    fn it_will_include_yesterdays_notes() {
        let standup = Standup::new().add_yesterday("yesterday");
        let json = serialize(&[&standup]);
        assert!(json.as_str().contains("yesterday\":[\"yesterday\"]"));
    }

    #[test]
    fn it_will_include_blocked_notes() {
        let standup = Standup::new().blocker("blocked");
        let json = serialize(&[&standup]);
        assert!(json.as_str().contains("blocked\":[\"blocked\"]"));
    }

    #[test]
    fn it_will_include_the_date() {
        let standup = Standup::from_date(Local.ymd(2015, 3, 23));
        let json = serialize(&[&standup]);
        assert!(json.as_str().contains("date\":\"2015-03-23\""));
    }
}
