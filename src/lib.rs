#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod splitter;

#[cfg(test)]
#[path = "./splitter/test.rs"]
mod test;
