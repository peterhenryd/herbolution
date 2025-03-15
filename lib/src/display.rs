use std::fmt::{Display, Formatter};

pub struct Join<'a, 'b, T>(pub &'a T, pub &'b str);

impl<'a, I> Display for Join<'a, '_, I>
where &'a I: IntoIterator,
      <&'a I as IntoIterator>::Item: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.into_iter();

        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
        }

        for item in iter {
            write!(f, "{}{}", self.1, item)?;
        }

        Ok(())
    }
}