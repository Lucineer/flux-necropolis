#[derive(Clone, Debug)]
pub struct MemorialVisit {
    pub visitor_id: u16,
    pub stone_index: usize,
    pub visit_time: u64,
    pub lessons_taken: u8,
}

pub struct MemorialLog {
    visits: Vec<MemorialVisit>,
}

impl MemorialLog {
    pub fn new() -> Self {
        Self { visits: Vec::new() }
    }

    pub fn record(&mut self, visitor: u16, stone: usize, lessons: u8) {
        self.visits.push(MemorialVisit {
            visitor_id: visitor,
            stone_index: stone,
            visit_time: 0,
            lessons_taken: lessons,
        });
    }

    pub fn visits_to(&self, stone: usize) -> usize {
        self.visits.iter().filter(|v| v.stone_index == stone).count()
    }

    pub fn visits_by(&self, visitor: u16) -> usize {
        self.visits.iter().filter(|v| v.visitor_id == visitor).count()
    }

    pub fn most_visited(&self, n: usize) -> Vec<(usize, usize)> {
        let mut counts: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for v in &self.visits {
            *counts.entry(v.stone_index).or_insert(0) += 1;
        }
        let mut pairs: Vec<(usize, usize)> = counts.into_iter().collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1));
        pairs.into_iter().take(n).collect()
    }

    pub fn total_visits(&self) -> usize {
        self.visits.len()
    }
}
