use super::Balancer;
use octopus_core::model::group::GroupItem;
use rand::Rng;

pub struct Weighted;

impl Weighted {
    pub fn new() -> Self { Self }
}

impl Balancer for Weighted {
    fn candidates(&self, items: &[GroupItem]) -> Vec<GroupItem> {
        if items.is_empty() { return vec![]; }
        let total_weight: i32 = items.iter().map(|i| i.weight.max(1)).sum();
        if total_weight <= 0 { return items.to_vec(); }

        let mut rng = rand::thread_rng();
        let mut remaining: Vec<&GroupItem> = items.iter().collect();
        let mut result = Vec::with_capacity(items.len());

        while !remaining.is_empty() {
            let current_weight: i32 = remaining.iter().map(|i| i.weight.max(1)).sum();
            let mut pick = rng.gen_range(0..current_weight);
            let mut idx = 0;
            for (i, item) in remaining.iter().enumerate() {
                pick -= item.weight.max(1);
                if pick < 0 { idx = i; break; }
            }
            result.push(remaining.remove(idx).clone());
        }
        result
    }
}