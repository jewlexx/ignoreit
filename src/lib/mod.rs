#[cfg(not(feature = "cache"))]
pub use crate::remote::get_templates;

#[cfg(feature = "cache")]
pub use crate::cache::get_templates;
