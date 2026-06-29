use super::{Balancer, shuffle_items};
use octopus_core::model::group::GroupItem;

pub struct RandomBalancer;

impl RandomBalancer {
    pub fn new() -> Self { Self }
}

impl Balancer for RandomBalancer {
    fn candidates(&self, items: &[GroupItem]) -> Vec<GroupItem> {
        shuffle_items(items)
    }
}