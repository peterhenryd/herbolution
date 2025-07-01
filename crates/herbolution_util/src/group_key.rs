use std::fmt::{Display, Formatter};
use std::hash::{DefaultHasher, Hash, Hasher};

use hashbrown::Equivalent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct GroupKeyBuf {
    pub group: String,
    pub key: String,
    hash: u64,
}

impl GroupKeyBuf {
    pub fn new(group: String, key: String) -> Self {
        let hash = hash_key(&group, &key);
        Self { group, key, hash }
    }

    pub fn as_ref(&self) -> GroupKey<'_> {
        GroupKey {
            group: &self.group,
            key: &self.key,
            hash: self.hash,
        }
    }

    pub fn len(&self) -> usize {
        self.group.len() + self.key.len() + 1
    }
}

impl Display for GroupKeyBuf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.group, self.key)
    }
}

impl Hash for GroupKeyBuf {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.hash.to_ne_bytes());
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GroupKey<'a> {
    pub group: &'a str,
    pub key: &'a str,
    hash: u64,
}

impl<'a> From<(&'a str, &'a str)> for GroupKey<'a> {
    fn from(value: (&'a str, &'a str)) -> Self {
        let (group, key) = value;
        let hash = hash_key(group, key);
        Self { group, key, hash }
    }
}

impl<'a> From<&'a GroupKeyBuf> for GroupKey<'a> {
    fn from(value: &'a GroupKeyBuf) -> Self {
        Self {
            group: &value.group,
            key: &value.key,
            hash: value.hash,
        }
    }
}

impl Hash for GroupKey<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.hash.to_ne_bytes());
    }
}

impl Equivalent<GroupKeyBuf> for GroupKey<'_> {
    fn equivalent(&self, other: &GroupKeyBuf) -> bool {
        self.group == &other.group && self.key == &other.key
    }
}

fn hash_key(group: &str, key: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    group.hash(&mut hasher);
    key.hash(&mut hasher);

    hasher.finish()
}

pub fn group_key(group: impl Into<String>, key: impl Into<String>) -> GroupKeyBuf {
    GroupKeyBuf::new(group.into(), key.into())
}
