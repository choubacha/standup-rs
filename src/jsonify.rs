use standup::{Standup, Aspect};
use chrono::*;
use serde_json::builder::ObjectBuilder;
use serde_json::{from_str,from_value,Value,Map};
use serde_json::error::Result;

type Obj = Map<String, Value>;

pub fn serialize(standups: &[&Standup]) -> String {
    let array = standups.iter().map(|standup| {
        ObjectBuilder::new()
            .insert("date", standup.date.format("%F").to_string())
            .insert("today", standup.today.clone())
            .insert("yesterday", standup.yesterday.clone())
            .insert("blocker", standup.blocker.clone())
            .unwrap()
    });
    format!("{}", Value::Array(array.collect::<Vec<Value>>()))
}

pub fn deserialize(json: String) -> Result<Vec<Standup>> {
    from_str(&json)
        .and_then(|parsed| from_value::<Vec<Obj>>(parsed))
        .map(|objs| objs.iter().map(|obj| build_standup(obj)).collect())
}

fn build_standup(obj: &Obj) -> Standup {
    let s = Standup::new();
    let s = add_message(s, &obj, "today",       |s, msg| s.add(Aspect::Today, msg));
    let s = add_message(s, &obj, "yesterday",   |s, msg| s.add(Aspect::Yesterday, msg));
    let s = add_message(s, &obj, "blocker",     |s, msg| s.add(Aspect::Blocker, msg));
    let s = set_date(s, &obj);
    s
}

fn add_message<F>(standup: Standup, obj: &Obj, key: &str, op: F) -> Standup
    where F: FnMut(Standup, &str) -> Standup
{
    obj.get(key)
        .and_then(|value| value.as_array())
        .map_or(standup.clone(), |messages| {
            messages
                .iter()
                .map(|value| value.as_string())
                .filter(|opt| opt.is_some())
                .map(|opt| opt.unwrap())
                .fold(standup, op)
        })
}

fn set_date(standup: Standup, obj: &Obj) -> Standup {
    obj.get("date")
        .and_then(|date_value| date_value.as_string())
        .map_or(standup.clone(), |date_string| {
            if let Ok(date) = NaiveDate::parse_from_str(date_string, "%F") {
                standup.set_date(Local.ymd(date.year(), date.month(), date.day()))
            } else {
                standup
            }
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use standup::{Standup, Aspect};
    use chrono::*;

    #[test]
    fn it_will_include_todays_notes() {
        let standup = Standup::new().add(Aspect::Today, "today");
        let json = serialize(&[&standup]);
        assert!(json.as_str().contains("today\":[\"today\"]"));
    }

    #[test]
    fn it_will_include_yesterdays_notes() {
        let standup = Standup::new().add(Aspect::Yesterday, "yesterday");
        let json = serialize(&[&standup]);
        assert!(json.as_str().contains("yesterday\":[\"yesterday\"]"));
    }

    #[test]
    fn it_will_include_blocker_notes() {
        let standup = Standup::new().add(Aspect::Blocker, "blocker");
        let json = serialize(&[&standup]);
        assert!(json.as_str().contains("blocker\":[\"blocker\"]"));
    }

    #[test]
    fn it_will_include_the_date() {
        let standup = Standup::from_date(Local.ymd(2015, 3, 23));
        let json = serialize(&[&standup]);
        assert!(json.as_str().contains("date\":\"2015-03-23\""));
    }

    #[test]
    fn it_will_load_in_todays_messages() {
        let standup = Standup::new().add(Aspect::Today, "today");
        let standups = deserialize(serialize(&[&standup])).unwrap();
        assert_eq!(standups[0].today, vec!["today"]);
    }

    #[test]
    fn it_will_load_in_yesterdays_messages() {
        let standup = Standup::new().add(Aspect::Yesterday, "yesterday");
        let standups = deserialize(serialize(&[&standup])).unwrap();
        assert_eq!(standups[0].yesterday, vec!["yesterday"]);
    }

    #[test]
    fn it_will_load_in_blockers() {
        let standup = Standup::new().add(Aspect::Blocker, "blocker");
        let standups = deserialize(serialize(&[&standup])).unwrap();
        assert_eq!(standups[0].blocker, vec!["blocker"]);
    }

    #[test]
    fn it_will_load_in_the_date() {
        let date = Local.ymd(2015, 3, 23);
        let standup = Standup::from_date(date.clone());
        let standups = deserialize(serialize(&[&standup])).unwrap();
        assert_eq!(standups[0].date, date);
    }
}
