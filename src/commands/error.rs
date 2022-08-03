use thiserror::Error as AsError;

#[derive(Debug, AsError)]
pub enum CommandError {}
