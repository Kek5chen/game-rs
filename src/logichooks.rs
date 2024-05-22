use std::error::Error;

use crate::world::World;

type HookFunc = fn(world: &mut World) -> Result<(), Box<dyn Error>>;

pub struct LogicHooks {
    pub init: Option<HookFunc>,
    pub update: Option<HookFunc>,
    pub deinit: Option<HookFunc>,
}
