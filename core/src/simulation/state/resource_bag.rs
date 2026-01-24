use crate::simulation::state::resource::ResourceType;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct ResourceBag {
    pub amounts: HashMap<ResourceType, (u64, f32)>,
}

impl ResourceBag {
    pub fn add(&mut self, resource: ResourceType, amount: f32) {
        let (total, frac) = self.amounts.entry(resource).or_insert((0, 0.0));

        *total += amount as u64;
        *frac += amount.fract();

        if *frac >= 1.0 {
            *frac -= 1.0;
            *total += 1;
        }
    }

    pub fn remove(&mut self, resource: ResourceType, amount: f32) -> bool {
        if self.total(resource) < amount {
            return false;
        }

        let (total, frac) = self.amounts.entry(resource).or_insert((0, 0.0));

        *frac -= amount.fract();
        *total -= amount as u64;

        if *frac < 0.0 {
            *frac += 1.0;
            *total -= 1;
        }

        true
    }

    pub fn total(&self, resource: ResourceType) -> f32 {
        self.amounts
            .get(&resource)
            .map_or(0.0, |(total, frac)| *total as f32 + frac)
    }
}
