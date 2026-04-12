#![allow(dead_code)]
#[derive(Clone,Debug,PartialEq)]
pub enum ArtKind { Function, Module, Test, Config, Documentation, Data, Skill }
pub struct Artifact { id: u32, name: String, kind: ArtKind, born: u32, died: u32, usefulness: f64, harvested: bool }
pub struct Necropolis { artifacts: Vec<Artifact>, catalog: Vec<Artifact>, next_id: u32, tick: u32 }

impl Necropolis {
    pub fn new() -> Self { Self { artifacts: Vec::new(), catalog: Vec::new(), next_id: 1, tick: 0 } }
    pub fn register(&mut self, name: &str, kind: ArtKind) -> u32 {
        let id = self.next_id; self.next_id += 1;
        self.artifacts.push(Artifact { id, name: name.to_string(), kind, born: self.tick, died: 0, usefulness: 0.5, harvested: false }); id
    }
    pub fn kill(&mut self, id: u32) -> Option<Artifact> {
        if let Some(a) = self.artifacts.iter_mut().find(|a| a.id == id) { a.died = self.tick; }
        let pos = self.artifacts.iter().position(|a| a.id == id);
        pos.map(|i| self.artifacts.remove(i))
    }
    pub fn harvest(&mut self, id: u32) -> bool {
        if let Some(a) = self.artifacts.iter_mut().find(|a| a.id == id && a.usefulness > 0.3) {
            a.harvested = true; let art = a.clone(); self.catalog.push(art); true
        } else { false }
    }
    pub fn catalog_count(&self) -> usize { self.catalog.len() }
    pub fn active_count(&self) -> usize { self.artifacts.iter().filter(|a| a.died == 0).count() }
    pub fn find(&self, id: u32) -> Option<&Artifact> { self.artifacts.iter().find(|a| a.id == id).or_else(|| self.catalog.iter().find(|a| a.id == id)) }
    pub fn by_kind(&self, kind: ArtKind) -> Vec<&Artifact> { self.artifacts.iter().filter(|a| a.kind == kind).collect() }
    pub fn tick_inc(&mut self) { self.tick += 1; }
    pub fn resurrect(&mut self, id: u32) -> Option<u32> {
        let pos = self.catalog.iter().position(|a| a.id == id)?; let mut art = self.catalog.remove(pos);
        let new_id = self.next_id; self.next_id += 1; art.id = new_id; art.born = self.tick; art.died = 0; art.harvested = false;
        self.artifacts.push(art); Some(new_id)
    }
    pub fn usefulness_of(&self, id: u32) -> f64 { self.find(id).map(|a| a.usefulness).unwrap_or(0.0) }
    pub fn rate(&mut self, id: u32, score: f64) { if let Some(a) = self.artifacts.iter_mut().find(|a| a.id == id) { a.usefulness = score.clamp(0.0, 1.0); } }
    pub fn prune(&mut self, max_age: u32) -> Vec<Artifact> {
        let old: Vec<Artifact> = self.artifacts.iter().filter(|a| a.died > 0 && self.tick - a.died > max_age && a.usefulness < 0.3).cloned().collect();
        self.artifacts.retain(|a| !(a.died > 0 && self.tick - a.died > max_age && a.usefulness < 0.3)); old
    }
    pub fn oldest(&self) -> Option<&Artifact> { self.artifacts.iter().filter(|a| a.died == 0).min_by_key(|a| a.born) }
    pub fn stats(&self) -> (usize, usize, usize) { (self.active_count(), self.catalog_count(), self.artifacts.iter().filter(|a| a.harvested).count()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_new() { let n = Necropolis::new(); assert_eq!(n.active_count(), 0); }
    #[test] fn test_register() { let mut n = Necropolis::new(); let id = n.register("foo", ArtKind::Function); assert!(id > 0); assert_eq!(n.active_count(), 1); }
    #[test] fn test_kill() { let mut n = Necropolis::new(); let id = n.register("foo", ArtKind::Module); n.kill(id); assert_eq!(n.active_count(), 0); }
    #[test] fn test_harvest() { let mut n = Necropolis::new(); let id = n.register("foo", ArtKind::Function); n.rate(id, 0.8); n.kill(id); assert!(n.harvest(id)); assert_eq!(n.catalog_count(), 1); }
    #[test] fn test_harvest_low_usefulness() { let mut n = Necropolis::new(); let id = n.register("foo", ArtKind::Data); n.rate(id, 0.1); n.kill(id); assert!(!n.harvest(id)); }
    #[test] fn test_resurrect() { let mut n = Necropolis::new(); let id = n.register("foo", ArtKind::Skill); n.rate(id, 0.8); n.kill(id); n.harvest(id); let new_id = n.resurrect(id); assert!(new_id.is_some()); assert_eq!(n.active_count(), 1); }
    #[test] fn test_by_kind() { let mut n = Necropolis::new(); n.register("a", ArtKind::Function); n.register("b", ArtKind::Module); assert_eq!(n.by_kind(ArtKind::Function).len(), 1); }
    #[test] fn test_prune() { let mut n = Necropolis::new(); let id = n.register("x", ArtKind::Data); n.rate(id, 0.1); n.kill(id); for _ in 0..20 { n.tick_inc(); } let p = n.prune(10); assert_eq!(p.len(), 1); }
    #[test] fn test_rate() { let mut n = Necropolis::new(); let id = n.register("x", ArtKind::Function); n.rate(id, 0.9); assert!((n.usefulness_of(id) - 0.9).abs() < 1e-6); }
    #[test] fn test_stats() { let mut n = Necropolis::new(); let id = n.register("x", ArtKind::Test); n.rate(id, 0.8); n.kill(id); n.harvest(id); let s = n.stats(); assert_eq!(s.1, 1); }
    #[test] fn test_oldest() { let mut n = Necropolis::new(); n.tick_inc(); let id1 = n.register("old", ArtKind::Function); n.tick_inc(); n.register("new", ArtKind::Function); assert_eq!(n.oldest().unwrap().id, id1); }
    #[test] fn test_find_catalog() { let mut n = Necropolis::new(); let id = n.register("x", ArtKind::Function); n.rate(id, 0.8); n.kill(id); n.harvest(id); assert!(n.find(id).is_some()); }
}