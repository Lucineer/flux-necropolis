use crate::tombstone::Tombstone;

pub const AFTERLIFE_MAX: usize = 128;

#[derive(Clone, Debug)]
pub struct KnowledgeFragment {
    pub key: String,
    pub value: String,
    pub source_vessel_id: u16,
    pub reused_count: u32,
}

pub struct Afterlife {
    fragments: Vec<KnowledgeFragment>,
}

impl Afterlife {
    pub fn new() -> Self {
        Self { fragments: Vec::new() }
    }

    pub fn store(&mut self, key: &str, value: &str, source_id: u16) -> Result<usize, &str> {
        if self.fragments.len() >= AFTERLIFE_MAX {
            return Err("afterlife is full");
        }
        let idx = self.fragments.len();
        self.fragments.push(KnowledgeFragment {
            key: key.to_string(),
            value: value.to_string(),
            source_vessel_id: source_id,
            reused_count: 0,
        });
        Ok(idx)
    }

    pub fn find(&self, key: &str) -> Option<&KnowledgeFragment> {
        self.fragments.iter().find(|f| f.key == key)
    }

    pub fn search(&self, prefix: &str, max: usize) -> Vec<&KnowledgeFragment> {
        self.fragments.iter().filter(|f| f.key.starts_with(prefix)).take(max).collect()
    }

    pub fn increment_reuse(&mut self, key: &str) {
        if let Some(f) = self.fragments.iter_mut().find(|f| f.key == key) {
            f.reused_count += 1;
        }
    }

    pub fn most_reused(&self, n: usize) -> Vec<&KnowledgeFragment> {
        let mut sorted: Vec<&KnowledgeFragment> = self.fragments.iter().collect();
        sorted.sort_by(|a, b| b.reused_count.cmp(&a.reused_count));
        sorted.into_iter().take(n).collect()
    }

    pub fn harvest(&mut self, stone: &Tombstone) {
        if !stone.lesson.is_empty() {
            let key = format!("v:{}:lesson", stone.vessel_id);
            if self.find(&key).is_none() {
                let _ = self.store(&key, &stone.lesson, stone.vessel_id);
            }
        }
    }

    pub fn by_source(&self, vessel_id: u16) -> Vec<&KnowledgeFragment> {
        self.fragments.iter().filter(|f| f.source_vessel_id == vessel_id).collect()
    }

    pub fn len(&self) -> usize {
        self.fragments.len()
    }
}
