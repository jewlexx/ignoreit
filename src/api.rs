use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::collections::HashMap;

pub type Gitignores = HashMap<String, Gitignore>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Gitignore {
    pub(crate) name: String,
    pub(crate) contents: String,
    pub(crate) file_name: String,
    pub(crate) key: String,
    #[sqlx(try_from = "i64")]
    pub(crate) is_patch: IsPatch,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub enum IsPatch {
    True,
    False,
}

impl From<i64> for IsPatch {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::False,
            1 => Self::True,
            _ => unreachable!(),
        }
    }
}

impl From<IsPatch> for bool {
    fn from(value: IsPatch) -> Self {
        match value {
            IsPatch::True => true,
            IsPatch::False => false,
        }
    }
}
