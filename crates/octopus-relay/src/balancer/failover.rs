use super::{Balancer, sort_by_priority};
use octopus_core::model::group::GroupItem;

pub struct Failover;

impl Failover {
    pub fn new() -> Self { Self }
}

impl Balancer for Failover {
    fn candidates(&self, items: &[GroupItem]) -> Vec<GroupItem> {
        sort_by_priority(items)
    }
}