#[cfg(not(feature = "cache"))]
pub use remote::get_templates;

#[cfg(feature = "cache")]
pub use cache::get_templates;
