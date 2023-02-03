pub struct Span {
    pub from: usize,
    pub to: usize,
}

pub trait Delimiter {
    fn get_next(&self, s: &str) -> Option<Span>;
}

impl Delimiter for char {
    fn get_next(&self, s: &str) -> Option<Span> {
        s.find(*self).map(|pos| Span {
            from: pos,
            to: pos + self.len_utf8(),
        })
    }
}

#[derive(Debug)]
pub struct StrSplit<'input, D> {
    remainder: Option<&'input str>,
    delimiter: D,
}

impl<'input, D> StrSplit<'input, D> {
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
            let res = self.remainder;
            self.remainder = None;

            res
        }
    }
}

#[cfg(test)]
#[path = "./test.rs"]
mod test;
