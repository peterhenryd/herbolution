use std::fmt::{Debug, Display, Formatter};
use std::mem::replace;

pub struct MemPrev<T> {
    prev: Option<T>,
    value: T,
}

impl<T> MemPrev<T> {
    pub const fn new(value: T) -> Self {
        Self { prev: None, value }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn get_prev(&self) -> Option<&T> {
        self.prev.as_ref()
    }

    pub fn get_prev_mut(&mut self) -> Option<&mut T> {
        self.prev.as_mut()
    }

    pub fn set(&mut self, value: T) {
        self.prev = Some(replace(&mut self.value, value));
    }
}

impl<T: Copy> Copy for MemPrev<T> {}

impl<T: Clone> Clone for MemPrev<T> {
    fn clone(&self) -> Self {
        Self {
            prev: self.prev.clone(),
            value: self.value.clone(),
        }
    }
}

impl<T: Debug> Debug for MemPrev<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemPrev")
            .field("prev", &self.prev)
            .field("value", &self.value)
            .finish()
    }
}

impl<T: Display> Display for MemPrev<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl<T: Default> Default for MemPrev<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
