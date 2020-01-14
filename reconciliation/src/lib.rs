#![type_length_limit = "1241512"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

#[macro_use]
pub mod macros;
pub mod system;
pub mod entity;
pub mod loader;
pub mod plugin;

pub mod prelude {
    pub use super::{
        system::*, declare_flush_plugin, entity::prelude::*, loader::prelude::*, plugin::prelude::*,
    };
}
