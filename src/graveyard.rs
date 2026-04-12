use crate::tombstone::{Tombstone, VesselState};

pub const GRAVEYARD_MAX: usize = 64;

pub struct Graveyard {
    stones: Vec<Tombstone>,
}

impl Graveyard {
    pub fn new() -> Self {
        Self {
            stones: Vec::new(),
        }
    }

    pub fn bury(&mut self, stone: Tombstone) -> Result<usize, &str> {
        if self.stones.len() >= GRAVEYARD_MAX {
            return Err("graveyard is full");
        }
        let idx = self.stones.len();
        self.stones.push(stone);
        Ok(idx)
    }

    pub fn find(&self, vessel_id: u16) -> Option<&Tombstone> {
        self.stones.iter().find(|s| s.vessel_id == vessel_id)
    }

    pub fn find_name(&self, name: &str) -> Option<&Tombstone> {
        self.stones.iter().find(|s| s.name == name)
    }

    pub fn find_mut(&mut self, vessel_id: u16) -> Option<&mut Tombstone> {
        self.stones.iter_mut().find(|s| s.vessel_id == vessel_id)
    }

    pub fn count_dead(&self) -> usize {
        self.stones.iter().filter(|s| s.state == VesselState::Dead).count()
    }

    pub fn count_memorialized(&self) -> usize {
        self.stones.iter().filter(|s| s.state == VesselState::Memorialized).count()
    }

    pub fn count_harvested(&self) -> usize {
        self.stones.iter().filter(|s| s.state == VesselState::Harvested).count()
    }

    pub fn recent_deaths(&self, n: usize) -> Vec<&Tombstone> {
        let mut dead: Vec<&Tombstone> = self.stones.iter().filter(|s| s.death_time > 0).collect();
        dead.sort_by(|a, b| b.death_time.cmp(&a.death_time));
        dead.into_iter().take(n).collect()
    }

    pub fn lessons(&self) -> Vec<&str> {
        self.stones.iter().filter(|s| s.state == VesselState::Dead).map(|s| s.lesson.as_str()).filter(|l| !l.is_empty()).collect()
    }

    pub fn vessels_by_trust(&self) -> Vec<&Tombstone> {
        let mut sorted: Vec<&Tombstone> = self.stones.iter().collect();
        sorted.sort_by(|a, b| b.peak_trust.partial_cmp(&a.peak_trust).unwrap_or(std::cmp::Ordering::Equal));
        sorted
    }

    pub fn find_index(&self, vessel_id: u16) -> Option<usize> {
        self.stones.iter().position(|s| s.vessel_id == vessel_id)
    }
}
