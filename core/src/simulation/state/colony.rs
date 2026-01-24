use crate::simulation::state::resource_bag::ResourceBag;

#[derive(Debug, Default, Clone)]
pub struct Colony {
    resources: ResourceBag,
}
