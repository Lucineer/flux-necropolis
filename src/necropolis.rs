use crate::afterlife::{Afterlife, KnowledgeFragment};
use crate::graveyard::Graveyard;
use crate::memorial::MemorialLog;
use crate::tombstone::{Tombstone, VesselState};

pub struct Necropolis {
    pub graveyard: Graveyard,
    pub afterlife: Afterlife,
    pub memorial: MemorialLog,
}

#[derive(Debug, Default)]
pub struct NecropolisStats {
    pub total_buried: usize,
    pub total_dead: usize,
    pub total_harvested: usize,
    pub total_knowledge: usize,
    pub total_memorial_visits: usize,
    pub oldest_tombstone_age_days: f64,
}

impl Necropolis {
    pub fn new() -> Self {
        Self {
            graveyard: Graveyard::new(),
            afterlife: Afterlife::new(),
            memorial: MemorialLog::new(),
        }
    }

    pub fn bury(&mut self, mut stone: Tombstone) -> Result<usize, &str> {
        self.afterlife.harvest(&stone);
        stone.knowledge_harvested = !stone.lesson.is_empty();
        if stone.state == VesselState::Dead && stone.knowledge_harvested {
            stone.state = VesselState::Harvested;
        }
        let idx = self.graveyard.bury(stone)?;
        Ok(idx)
    }

    pub fn visit(&mut self, visitor: u16, vessel_id: u16, lessons: u8) -> bool {
        if self.graveyard.find(vessel_id).is_none() {
            return false;
        }
        if let Some(pos) = self.graveyard.find_index(vessel_id) {
            self.memorial.record(visitor, pos, lessons);
            true
        } else {
            false
        }
    }

    pub fn search_wisdom(&self, query: &str) -> Vec<&KnowledgeFragment> {
        self.afterlife.search(query, 16)
    }

    pub fn statistics(&self) -> NecropolisStats {
        NecropolisStats {
            total_buried: self.graveyard.count_dead() + self.graveyard.count_memorialized() + self.graveyard.count_harvested(),
            total_dead: self.graveyard.count_dead(),
            total_harvested: self.graveyard.count_harvested(),
            total_knowledge: self.afterlife.len(),
            total_memorial_visits: self.memorial.total_visits(),
            oldest_tombstone_age_days: 0.0,
        }
    }
}
