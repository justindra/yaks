// Core business logic - independent of infrastructure
// Contains Yak model, validation rules, and domain operations

pub mod yak;

pub use yak::{Yak, validate_yak_name};
