use std::sync::atomic::{AtomicU64, Ordering};
use super::Balancer;
use octopus_core::model::group::GroupItem;

pub struct RoundRobin {
    counter: AtomicU64,
}

impl RoundRobin {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
        }
    }
}

impl Balancer for RoundRobin {
    fn candidates(&self, items: &[GroupItem]) -> Vec<GroupItem> {
        let n = items.len();
        if n == 0 {
            return vec![];
        }
        let idx = self.counter.fetch_add(1, Ordering::Relaxed) as usize % n;
        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            result.push(items[(idx + i) % n].clone());
        }
        result
    }
}