use std::iter::Enumerate;
use std::slice::Iter;
use std::sync::Arc;

use fontdue::Font;

pub struct Fonts {
    vec: Vec<MultiSizeFont>,
    filter_char: Box<dyn Fn(char) -> bool>,
}

impl Fonts {
    pub fn build() -> FontsBuilder {
        FontsBuilder::new()
    }

    pub fn get(&self, id: FontId) -> Option<&MultiSizeFont> {
        self.vec.get(id.index)
    }

    pub fn iter(&self) -> FontsIter<'_> {
        FontsIter {
            iter: self.vec.iter().enumerate(),
        }
    }

    pub fn has_char(&self, c: char) -> bool {
        (self.filter_char)(c)
    }
}

pub struct FontsIter<'a> {
    iter: Enumerate<Iter<'a, MultiSizeFont>>,
}

impl<'a> Iterator for FontsIter<'a> {
    type Item = (FontId, &'a MultiSizeFont);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, font)| (FontId { index }, font))
    }
}

pub struct FontsBuilder {
    vec: Vec<MultiSizeFont>,
    filter_char: Box<dyn Fn(char) -> bool>,
}

impl FontsBuilder {
    pub fn new() -> Self {
        FontsBuilder {
            vec: Vec::new(),
            filter_char: Box::new(|_| true),
        }
    }

    pub fn set_filter(&mut self, filter: impl Fn(char) -> bool + 'static) {
        self.filter_char = Box::new(filter);
    }

    pub fn add_font<const N: usize>(&mut self, font: Font, sizes: [f32; N]) -> FontId {
        let index = self.vec.len();
        let font = MultiSizeFont(Arc::new(MsfInner { font, sizes }));
        self.vec.push(font);
        FontId { index }
    }

    pub fn finish(self) -> Fonts {
        Fonts {
            vec: self.vec,
            filter_char: Box::new(|_| true),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FontId {
    index: usize,
}

#[derive(Debug, Clone)]
pub struct MultiSizeFont(Arc<MsfInner<[f32]>>);

impl MultiSizeFont {
    pub fn font(&self) -> &Font {
        &self.0.font
    }

    pub fn sizes(&self) -> &[f32] {
        &self.0.sizes
    }
}

#[derive(Debug)]
struct MsfInner<S: ?Sized> {
    font: Font,
    sizes: S,
}
