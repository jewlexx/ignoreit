use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Gitignores(HashMap<String, Gitignore>);

impl Gitignores {
    /// See [`Gitignore::normalise`]
    pub fn normalise(&mut self) {
        for (_key, value) in self.0.iter_mut() {
            value.normalise();
        }
    }
}

impl Deref for Gitignores {
    type Target = HashMap<String, Gitignore>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Gitignores {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Gitignore {
    pub(crate) name: String,
    pub(crate) contents: String,
    pub(crate) file_name: String,
    pub(crate) key: String,
    #[sqlx(try_from = "i64")]
    #[serde(skip)]
    pub(crate) is_patch: IsPatch,
}

impl Gitignore {
    /// Normalises the [`Gitignore`] type structure
    ///
    /// Currently does the following:
    /// - Checks for patch name schemes and marks it as a patch if that scheme is present
    pub fn normalise(&mut self) {
        if let Some(plus_pos) = self.name.find('+')
            && plus_pos != self.name.len() - 1
        {
            self.is_patch = true.into();
        } else {
            self.is_patch = false.into();
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub enum IsPatch {
    True,
    #[default]
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

impl From<IsPatch> for i64 {
    fn from(value: IsPatch) -> Self {
        match value {
            IsPatch::True => 1,
            IsPatch::False => 0,
        }
    }
}

impl From<bool> for IsPatch {
    fn from(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
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
