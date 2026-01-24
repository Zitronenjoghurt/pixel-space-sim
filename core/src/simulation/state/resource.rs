use crate::math::rgba::RGBA;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ResourceType {
    Ice,
    Iron,
    Gold,
}

impl From<ResourceType> for RGBA {
    fn from(resource_type: ResourceType) -> Self {
        match resource_type {
            ResourceType::Ice => RGBA::rgb(98, 194, 207),
            ResourceType::Iron => RGBA::rgb(165, 95, 75),
            ResourceType::Gold => RGBA::rgb(207, 179, 84),
        }
    }
}
