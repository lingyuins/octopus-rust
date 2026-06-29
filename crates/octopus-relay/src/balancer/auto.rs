use super::Balancer;
use octopus_core::model::group::GroupItem;

pub struct Auto;

impl Auto {
    pub fn new() -> Self { Self }
}

impl Balancer for Auto {
    fn candidates(&self, items: &[GroupItem]) -> Vec<GroupItem> {
        // Auto strategy: explore first, then prefer higher success rate
        // For now, fall back to shuffle
        if items.is_empty() { return vec![]; }
        let result = items.to_vec();
        super::shuffle_items(&result)
    }
}