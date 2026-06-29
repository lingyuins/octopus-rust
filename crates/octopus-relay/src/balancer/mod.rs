pub mod round_robin;
pub mod random;
pub mod failover;
pub mod weighted;
pub mod auto;
pub mod runtime_state;

use octopus_core::model::group::GroupItem;
use octopus_core::types::GroupMode;
use rand::seq::SliceRandom;

/// Balancer trait - selects and orders channel candidates
pub trait Balancer: Send + Sync {
    fn candidates(&self, items: &[GroupItem]) -> Vec<GroupItem>;
}

/// Get the appropriate balancer for a given mode
pub fn get_balancer(mode: GroupMode) -> Box<dyn Balancer> {
    match mode {
        GroupMode::RoundRobin => Box::new(round_robin::RoundRobin::new()),
        GroupMode::Random => Box::new(random::RandomBalancer::new()),
        GroupMode::Failover => Box::new(failover::Failover::new()),
        GroupMode::Weighted => Box::new(weighted::Weighted::new()),
        GroupMode::Auto => Box::new(auto::Auto::new()),
    }
}

/// Sort items by priority (ascending)
pub fn sort_by_priority(items: &[GroupItem]) -> Vec<GroupItem> {
    let mut sorted = items.to_vec();
    sorted.sort_by_key(|item| item.priority);
    sorted
}

/// Shuffle items randomly
pub fn shuffle_items(items: &[GroupItem]) -> Vec<GroupItem> {
    let mut rng = rand::thread_rng();
    let mut shuffled = items.to_vec();
    shuffled.shuffle(&mut rng);
    shuffled
}