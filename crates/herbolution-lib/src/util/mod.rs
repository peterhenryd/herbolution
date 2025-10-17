use std::fmt::{Display, Formatter};

crate::reexport! {
    mod group_key;
    mod time;
}

pub fn default<T: Default>() -> T {
    T::default()
}

pub struct JointDisplay<'i, 's, I> {
    iter: &'i I,
    sep: &'s str,
}

impl<'i, T: Display, I> Display for JointDisplay<'i, '_, I>
where
    &'i I: IntoIterator<Item = T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter.into_iter();

        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
        }

        for item in iter {
            write!(f, "{}{item}", self.sep)?;
        }

        Ok(())
    }
}

pub trait DisplayJoined {
    fn display_joined<'i, 's>(&'i self, sep: &'s str) -> JointDisplay<'i, 's, Self>
    where
        Self: Sized;
}

impl<I, T> DisplayJoined for I
where
    I: IntoIterator<Item = T>,
    T: Display,
{
    fn display_joined<'i, 's>(&'i self, sep: &'s str) -> JointDisplay<'i, 's, Self> {
        JointDisplay { iter: self, sep }
    }
}

#[macro_export]
macro_rules! reexport {
    (
    $(
        $vis:vis mod $name:ident;
    )+
    ) => {
    $(
        $vis mod $name;
    )+

    $(
        pub use $name::*;
    )+
    }
}
