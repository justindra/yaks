// Core business logic - independent of infrastructure
// Contains Yak model, validation rules, and domain operations

pub mod field;
pub mod yak;

pub use field::{validate_field_name, CONTEXT_FIELD, STATE_FIELD};
pub use yak::{validate_yak_name, Yak};
