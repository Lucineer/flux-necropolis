pub mod afterlife;
pub mod graveyard;
pub mod memorial;
pub mod necropolis;
pub mod tombstone;

pub use afterlife::{Afterlife, KnowledgeFragment};
pub use graveyard::Graveyard;
pub use memorial::{MemorialLog, MemorialVisit};
pub use necropolis::{Necropolis, NecropolisStats};
pub use tombstone::{Tombstone, VesselState};

#[cfg(test)]
mod tests {
    use super::*;

    // 1. tombstone new defaults
    #[test]
    fn tombstone_new_defaults() {
        let t = Tombstone::new(1, "TestVessel");
        assert_eq!(t.vessel_id, 1);
        assert_eq!(t.name, "TestVessel");
        assert_eq!(t.state, VesselState::Alive);
        assert!(t.cause.is_empty());
        assert!(t.lesson.is_empty());
        assert_eq!(t.commits_made, 0);
        assert_eq!(t.peak_trust, 0.0);
    }

    // 2. tombstone mark_dead
    #[test]
    fn tombstone_mark_dead() {
        let mut t = Tombstone::new(2, "Vessel2");
        t.birth_time = 100;
        t.mark_dead(200);
        assert_eq!(t.state, VesselState::Dead);
        assert_eq!(t.death_time, 200);
        assert_eq!(t.cycles_lived, 100);
        assert!((t.lifetime_days() - 100.0 / 86400.0).abs() < f64::EPSILON);
    }

    // 3. graveyard bury and find
    #[test]
    fn graveyard_bury_and_find() {
        let mut gy = Graveyard::new();
        let t = Tombstone::new(10, "Vessel10");
        let idx = gy.bury(t).unwrap();
        assert_eq!(idx, 0);
        assert!(gy.find(10).is_some());
        assert!(gy.find(99).is_none());
    }

    // 4. graveyard find_name
    #[test]
    fn graveyard_find_name() {
        let mut gy = Graveyard::new();
        gy.bury(Tombstone::new(1, "Alpha")).unwrap();
        gy.bury(Tombstone::new(2, "Beta")).unwrap();
        assert!(gy.find_name("Alpha").is_some());
        assert!(gy.find_name("Gamma").is_none());
    }

    // 5. graveyard counts
    #[test]
    fn graveyard_counts() {
        let mut gy = Graveyard::new();
        let mut t1 = Tombstone::new(1, "A");
        t1.state = VesselState::Dead;
        let mut t2 = Tombstone::new(2, "B");
        t2.state = VesselState::Memorialized;
        let mut t3 = Tombstone::new(3, "C");
        t3.state = VesselState::Harvested;
        gy.bury(t1).unwrap();
        gy.bury(t2).unwrap();
        gy.bury(t3).unwrap();
        assert_eq!(gy.count_dead(), 1);
        assert_eq!(gy.count_memorialized(), 1);
        assert_eq!(gy.count_harvested(), 1);
    }

    // 6. graveyard recent_deaths
    #[test]
    fn graveyard_recent_deaths() {
        let mut gy = Graveyard::new();
        let mut t1 = Tombstone::new(1, "A");
        t1.death_time = 100;
        let mut t2 = Tombstone::new(2, "B");
        t2.death_time = 200;
        gy.bury(t1).unwrap();
        gy.bury(t2).unwrap();
        let recent = gy.recent_deaths(1);
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].vessel_id, 2);
    }

    // 7. graveyard lessons
    #[test]
    fn graveyard_lessons() {
        let mut gy = Graveyard::new();
        let mut t = Tombstone::new(1, "A");
        t.state = VesselState::Dead;
        t.lesson = "always check types".to_string();
        gy.bury(t).unwrap();
        let lessons = gy.lessons();
        assert_eq!(lessons.len(), 1);
        assert_eq!(lessons[0], "always check types");
    }

    // 8. graveyard vessels_by_trust
    #[test]
    fn graveyard_vessels_by_trust() {
        let mut gy = Graveyard::new();
        let mut t1 = Tombstone::new(1, "A");
        t1.peak_trust = 0.5;
        let mut t2 = Tombstone::new(2, "B");
        t2.peak_trust = 0.9;
        gy.bury(t1).unwrap();
        gy.bury(t2).unwrap();
        let sorted = gy.vessels_by_trust();
        assert_eq!(sorted[0].vessel_id, 2);
        assert_eq!(sorted[1].vessel_id, 1);
    }

    // 9. afterlife store and find
    #[test]
    fn afterlife_store_and_find() {
        let mut al = Afterlife::new();
        al.store("key1", "value1", 1).unwrap();
        assert!(al.find("key1").is_some());
        assert_eq!(al.find("key1").unwrap().value, "value1");
        assert!(al.find("missing").is_none());
    }

    // 10. afterlife search prefix
    #[test]
    fn afterlife_search_prefix() {
        let mut al = Afterlife::new();
        al.store("rust:tip1", "use iterators", 1).unwrap();
        al.store("rust:tip2", "clippy is good", 1).unwrap();
        al.store("go:tip1", "handle errors", 2).unwrap();
        let results = al.search("rust:", 10);
        assert_eq!(results.len(), 2);
    }

    // 11. afterlife reuse increment
    #[test]
    fn afterlife_reuse_increment() {
        let mut al = Afterlife::new();
        al.store("key1", "val1", 1).unwrap();
        al.increment_reuse("key1");
        al.increment_reuse("key1");
        assert_eq!(al.find("key1").unwrap().reused_count, 2);
    }

    // 12. afterlife most_reused
    #[test]
    fn afterlife_most_reused() {
        let mut al = Afterlife::new();
        al.store("a", "1", 1).unwrap();
        al.store("b", "2", 1).unwrap();
        al.increment_reuse("b");
        al.increment_reuse("b");
        al.increment_reuse("b");
        let top = al.most_reused(1);
        assert_eq!(top[0].key, "b");
    }

    // 13. afterlife harvest from tombstone
    #[test]
    fn afterlife_harvest_from_tombstone() {
        let mut al = Afterlife::new();
        let mut t = Tombstone::new(5, "Harvested");
        t.lesson = "don't panic".to_string();
        al.harvest(&t);
        assert!(al.find("v:5:lesson").is_some());
        assert_eq!(al.find("v:5:lesson").unwrap().value, "don't panic");
    }

    // 14. memorial record and counts
    #[test]
    fn memorial_record_and_counts() {
        let mut ml = MemorialLog::new();
        ml.record(1, 0, 2);
        ml.record(1, 0, 1);
        ml.record(2, 0, 1);
        assert_eq!(ml.visits_to(0), 3);
        assert_eq!(ml.visits_by(1), 2);
        let top = ml.most_visited(1);
        assert_eq!(top[0], (0, 3));
    }

    // 15. necropolis bury auto-harvests
    #[test]
    fn necropolis_bury_auto_harvests() {
        let mut nec = Necropolis::new();
        let mut t = Tombstone::new(1, "Auto");
        t.mark_dead(100);
        t.set_lesson("automated wisdom");
        nec.bury(t).unwrap();
        assert!(nec.afterlife.find("v:1:lesson").is_some());
    }

    // 16. necropolis visit
    #[test]
    fn necropolis_visit() {
        let mut nec = Necropolis::new();
        let mut t = Tombstone::new(1, "Visited");
        t.mark_dead(50);
        nec.bury(t).unwrap();
        assert!(nec.visit(10, 1, 3));
        assert!(!nec.visit(10, 99, 1)); // non-existent
    }

    // 17. necropolis statistics
    #[test]
    fn necropolis_statistics() {
        let mut nec = Necropolis::new();
        let mut t = Tombstone::new(1, "Stats");
        t.mark_dead(100);
        t.set_lesson("stats lesson");
        nec.bury(t).unwrap();
        nec.visit(5, 1, 1);
        let stats = nec.statistics();
        assert_eq!(stats.total_buried, 1);
        assert_eq!(stats.total_knowledge, 1);
        assert_eq!(stats.total_memorial_visits, 1);
    }

    // 18. graveyard full error
    #[test]
    fn graveyard_full_error() {
        let mut gy = Graveyard::new();
        for i in 0..64 {
            gy.bury(Tombstone::new(i as u16, &format!("v{}", i))).unwrap();
        }
        let result = gy.bury(Tombstone::new(99, "overflow"));
        assert!(result.is_err());
    }
}
