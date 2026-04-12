#[derive(Clone, Debug, PartialEq)]
pub enum VesselState {
    Alive,
    Dying,
    Dead,
    Memorialized,
    Harvested,
}

#[derive(Clone, Debug)]
pub struct Tombstone {
    pub vessel_id: u16,
    pub name: String,
    pub state: VesselState,
    pub cause: String,
    pub lesson: String,
    pub birth_time: u64,
    pub death_time: u64,
    pub cycles_lived: u64,
    pub commits_made: u32,
    pub repos_touched: u32,
    pub peak_trust: f32,
    pub avg_confidence: f32,
    pub knowledge_harvested: bool,
}

impl Tombstone {
    pub fn new(id: u16, name: &str) -> Self {
        Self {
            vessel_id: id,
            name: name.to_string(),
            state: VesselState::Alive,
            cause: String::new(),
            lesson: String::new(),
            birth_time: 0,
            death_time: 0,
            cycles_lived: 0,
            commits_made: 0,
            repos_touched: 0,
            peak_trust: 0.0,
            avg_confidence: 0.0,
            knowledge_harvested: false,
        }
    }

    pub fn set_cause(&mut self, cause: &str) {
        self.cause = cause.to_string();
    }

    pub fn set_lesson(&mut self, lesson: &str) {
        self.lesson = lesson.to_string();
    }

    pub fn mark_dead(&mut self, time: u64) {
        self.state = VesselState::Dead;
        self.death_time = time;
        if self.birth_time < time {
            self.cycles_lived = time - self.birth_time;
        }
    }

    pub fn age_secs(&self, now: u64) -> u64 {
        if self.death_time > 0 && now >= self.death_time {
            now - self.death_time
        } else {
            0
        }
    }

    pub fn lifetime_days(&self) -> f64 {
        self.cycles_lived as f64 / 86400.0
    }
}
