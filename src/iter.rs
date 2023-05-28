use crate::enums::CSISequence;
use crate::parser::{Match, parse};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Output<'a> {
    Text(&'a str),
    Escape(CSISequence),
}

impl Output<'_> {
    #[inline]
    pub fn is_text(&self) -> bool {
        match self {
            Output::Text(_) => true,
            Output::Escape(_) => false,
        }
    }

    #[inline]
    pub fn is_esc(&self) -> bool {
        !self.is_text()
    }
}


/// Once this trait is implemented, we can parse CSI and implement an iterator.
/// ```
/// #[cfg(all(not(feature = "std"), feature = "alloc"))]
/// impl CsiParser for String {
///     fn csi_parser(&self) -> CsiIterator {
///         CsiIterator {
///            data: self,
///            matches: Some(parse(self)),
///            index: 0,
///            index_of_data: 0,
///         }
///     }
/// }
/// ```
pub trait CsiParser {
    fn csi_parser(&self) -> CsiIterator;
}

impl CsiParser for str {
    fn csi_parser(&self) -> CsiIterator {
        CsiIterator {
            data: self,
            matches: Some(parse(self)),
            index: 0,
            index_of_data: 0,
        }
    }
}

/// the csi iterator
pub struct CsiIterator<'a> {
    // original str
    data: &'a str,
    // the matches of the data
    #[cfg(all(not(feature = "std"), feature = "no_std"))]
    matches: Option<alloc::vec::Vec<Match<'a>>>,
    #[cfg(feature = "std")]
    matches: Option<std::vec::Vec<Match<'a>>>,
    // csi seq index
    index: usize,
    // the index of the data
    index_of_data: usize,
}

impl<'a> Iterator for CsiIterator<'a> {
    type Item = Output<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }

        // 获取所有的CSI,初始化数据
        if self.matches.is_none() {
            self.matches = Some(parse(self.data));
            self.index = 0;
            self.index_of_data = 0;
        }

        if let Some(matches) = &self.matches {
            if self.index < matches.len() {
                let item = &matches[self.index];

                return if item.start > self.index_of_data {
                    // process before csi sequence data
                    let out = Some(Output::Text(&self.data[self.index_of_data..item.start]));
                    self.index_of_data = item.start;
                    out
                } else {
                    self.index_of_data = item.end;
                    // now we can process the next csi sequence
                    self.index += 1;
                    // process the csi sequence data
                    Some(Output::Escape(item.into()))
                };
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::enums::CSISequence::Color;
    use super::*;

    #[test]
    fn test_iter() {
        let text = "\x1b[31mhello,world\x1b[m";
        let out: Vec<Output> = text.csi_parser().skip(1).collect();

        assert_eq!(out, vec![Output::Text("hello,world"),
                             Output::Escape(Color(None, None, None))]);
    }

    #[test]
    fn test_filter() {
        let text = "\x1b[31mhello,world\x1b[m";
        let out: Vec<Output> = text.csi_parser().filter(Output::is_text).collect();

        assert_eq!(out, vec![Output::Text("hello,world")]);
    }
}