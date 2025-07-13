use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::alloc::{alloc, handle_alloc_error, Layout};
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::{ptr, slice};

#[derive(Debug, Eq)]
pub struct GroupKey {
    sep: usize,
    str: str,
}

impl GroupKey {
    #[inline]
    pub fn group(&self) -> &str {
        &self.str[..self.sep]
    }

    #[inline]
    pub fn key(&self) -> &str {
        &self.str[self.sep + 1..]
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.str
    }
}

impl PartialEq for GroupKey {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.str == other.str
    }
}

impl Hash for GroupKey {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl Display for GroupKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct GroupKeyBuf(Box<GroupKey>);

impl GroupKeyBuf {
    pub fn new(group: &str, key: &str) -> Self {
        let sep = group.len();
        let len = sep + key.len() + 1;

        let (layout, offset) = layout(len);

        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            handle_alloc_error(layout);
        }

        unsafe {
            ptr.cast::<usize>().write(sep);
        }

        let slice = unsafe { slice::from_raw_parts_mut(ptr.add(offset), len) };
        slice[0..sep].copy_from_slice(group.as_bytes());
        slice[sep] = b':';
        slice[sep + 1..].copy_from_slice(key.as_bytes());

        Self(unsafe { Box::from_raw(ptr::from_raw_parts_mut(ptr, len)) })
    }
}

impl Hash for GroupKeyBuf {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

fn layout(str_len: usize) -> (Layout, usize) {
    let sep_layout = Layout::new::<u64>();
    let str_layout = Layout::from_size_align(str_len, 1).unwrap();

    let (layout, offset) = sep_layout.extend(str_layout).unwrap();
    let layout = layout.pad_to_align();

    (layout, offset)
}

impl Clone for GroupKeyBuf {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.group(), self.key())
    }
}

impl Deref for GroupKeyBuf {
    type Target = GroupKey;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for GroupKeyBuf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let De { sep, string } = De::deserialize(deserializer)?;

        Ok(Self::new(&string[..sep], &string[sep + 1..]))
    }
}

impl Serialize for GroupKeyBuf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ser { sep: self.sep, str: &self.str }.serialize(serializer)
    }
}

impl Borrow<GroupKey> for GroupKeyBuf {
    #[inline]
    fn borrow(&self) -> &GroupKey {
        self.deref()
    }
}

impl Borrow<str> for GroupKeyBuf {
    #[inline]
    fn borrow(&self) -> &str {
        &self.str
    }
}

#[derive(Serialize)]
struct Ser<'a> {
    sep: usize,
    str: &'a str,
}

#[derive(Deserialize)]
struct De {
    sep: usize,
    string: String,
}
