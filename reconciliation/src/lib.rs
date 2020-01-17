#![type_length_limit = "1241512"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

#[macro_use]
mod macros;
mod entity;
mod excel;
mod loader;
mod plugin;
mod system;

pub mod prelude {
    pub use super::{
        declare_flush_plugin, entity::prelude::*, excel::*, loader::prelude::*, plugin::prelude::*,
        system::*,
    };
}
