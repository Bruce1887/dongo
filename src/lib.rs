// modules in src/
pub mod common;
pub mod error;
pub mod event_handler;
pub mod map_generator;
pub mod shapes;

//modules in folders under src/
pub mod user_control;
use user_control::*;

// pub mod old_dongo_entity;
// pub use old_dongo_entity::*;

pub mod dongo_entity;
pub use dongo_entity::*;

pub mod data_massage_parlor;
