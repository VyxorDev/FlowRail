//! Normalized activity records; providers stay separate from the interface.

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Priority {
    Urgent,
    Active,
    Background,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Activity {
    pub id: &'static str,
    pub title: String,
    pub detail: String,
    pub priority: Priority,
}

#[derive(Default)]
pub struct ActivityFeed {
    entries: Vec<Activity>,
}

impl ActivityFeed {
    pub fn upsert(&mut self, item: Activity) {
        if let Some(existing) = self.entries.iter_mut().find(|a| a.id == item.id) {
            *existing = item;
        } else {
            self.entries.push(item);
        }
    }

    pub fn visible(&self, quiet_mode: bool) -> Vec<&Activity> {
        let mut entries: Vec<&Activity> = self
            .entries
            .iter()
            .filter(|entry| !quiet_mode || entry.priority != Priority::Background)
            .collect();
        entries.sort_by_key(|entry| entry.priority);
        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: &'static str, priority: Priority) -> Activity {
        Activity { id, title: id.into(), detail: String::new(), priority }
    }

    #[test]
    fn quiet_mode_removes_background_only() {
        let mut feed = ActivityFeed::default();
        feed.upsert(item("bg", Priority::Background));
        feed.upsert(item("focus", Priority::Active));
        feed.upsert(item("alert", Priority::Urgent));
        let ids: Vec<_> = feed.visible(true).into_iter().map(|entry| entry.id).collect();
        assert_eq!(ids, ["alert", "focus"]);
    }

    #[test]
    fn upsert_replaces_duplicate_ids() {
        let mut feed = ActivityFeed::default();
        feed.upsert(item("focus", Priority::Background));
        feed.upsert(item("focus", Priority::Urgent));
        assert_eq!(feed.visible(false).len(), 1);
        assert_eq!(feed.visible(false)[0].priority, Priority::Urgent);
    }
}
