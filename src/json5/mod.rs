


mod de;
mod error;

mod deserialize_item;
mod test;
mod jval;

pub use crate::json5::de::from_str;
pub use crate::json5::jval::{JVal, Span};
pub use crate::json5::error::MyError;

