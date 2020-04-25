use uuid::Uuid;
use std::time::Duration;

#[derive(Clone)]
pub struct Pomodoro {
    id: Uuid,
    duration: Duration,
    name: String,
    tags: Vec<Tag>,
}

impl Pomodoro {
    pub fn new(
        name: String,
        duration: Duration,
        tags: Vec<Tag>,
    ) -> Self {
        Pomodoro {
            id: Uuid::new_v4(),
            duration,
            name,
            tags,
        }
    }
}

#[derive(Clone)]
pub struct Tag {
    id: Uuid,
    name: String,
}

impl Tag {
    pub fn new(name: String) -> Self {
        Tag { id: Uuid::new_v4(), name }
    }
}
