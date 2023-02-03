// ----------------------------------------------------------------------------

pub struct Span {
    pub from: usize,
    pub to: usize,
}

// EXPERIMENTAL: This trait is a common length calculator.
//               Idea was to try and create a default impl of get_next()
//               Input needs to be `find'-able for this
// NOT REALLY A BIG SAVING or CLEARER 🤕

pub trait DelimLen {
    fn dlen(&self) -> usize;
}

impl DelimLen for char {
    fn dlen(&self) -> usize {
        self.len_utf8()
    }
}

impl DelimLen for &str {
    fn dlen(&self) -> usize {
        self.len()
    }
}

pub trait Finder {
    fn find(&self, s: &str) -> Option<usize>;
}

impl Finder for char {
    fn find(&self, s: &str) -> Option<usize> {
        s.find(*self)
    }
}

impl Finder for &str {
    fn find(&self, s: &str) -> Option<usize> {
        s.find(*self)
    }
}

pub trait Delimiter: DelimLen + Finder {
    fn get_next(&self, s: &str) -> Option<Span> {
        self.find(s).map(|pos| Span {
            from: pos,
            to: pos + self.dlen(),
        })
    }
}

impl Delimiter for char {}
impl Delimiter for &str {}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct StrSplit<'input, D> {
    remainder: Option<&'input str>,
    delimiter: D,
}

impl<'input, D> StrSplit<'input, D>
where
    D: Delimiter,
{
    pub fn new(input: &'input str, delimiter: D) -> Self {
        Self {
            remainder: Some(input),
            delimiter,
        }
    }
}

impl<'input, D> Iterator for StrSplit<'input, D>
where
    D: Delimiter,
{
    type Item = &'input str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(range) = self.delimiter.get_next(&remainder) {
            let res = &remainder[..range.from];
            *remainder = &remainder[range.to..];

            Some(res)
        } else {
            self.remainder.take()

            // NOTE: Above is idiomatic for the following
            // let res = self.remainder;
            // self.remainder = None;
            // res
        }
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c).next().unwrap_or("!")
}

#[cfg(test)]
#[path = "./splitter/test.rs"]
mod test;
