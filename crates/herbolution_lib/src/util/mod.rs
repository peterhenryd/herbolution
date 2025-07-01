pub mod display;
pub mod group_key;
pub mod time;

pub fn default<T: Default>() -> T {
    T::default()
}
