use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::collections::HashMap;

pub type Gitignores = HashMap<String, Gitignore>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Gitignore {
    pub(crate) name: String,
    pub(crate) contents: String,
    pub(crate) file_name: String,
    pub(crate) key: String,
}
