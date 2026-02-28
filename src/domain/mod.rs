mod aggregate;
mod error;
mod event;
pub mod query;
pub mod value_objects;

pub use aggregate::AccountState;
pub use error::DomainError;
pub use event::AccountEvent;
pub use value_objects::{Account, Amount, Category, Description};
