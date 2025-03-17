use std::fmt::{Display, Formatter};

pub struct Join<'a, 'b, I> {
    pub iter: &'a I,
    pub separator: &'b str,
}

impl<'a, 'b, I> Join<'a, 'b, I> {
    pub fn new(iter: &'a I, separator: &'b str) -> Self {
        Self {
            iter,
            separator,
        }
    }
}

impl<'a, I> Display for Join<'a, '_, I>
where &'a I: IntoIterator,
      <&'a I as IntoIterator>::Item: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter.into_iter();

        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
        }

        for item in iter {
            write!(f, "{}{}", self.separator, item)?;
        }

        Ok(())
    }
}