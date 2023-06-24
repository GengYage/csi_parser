use crate::enums::CSISequence;
use crate::parser::{parse, Match};

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

        if let Some(csi_matches) = &self.matches {
            // 解析出来的csi序列不存在,则全部是文本
            if csi_matches.is_empty() {
                // 用index标识是否遍历完成
                return if self.index == 0 {
                    self.index += 1;
                    Some(Output::Text(self.data))
                } else {
                    None
                };
            }

            return if self.index < csi_matches.len() {
                let csi_item = &csi_matches[self.index];

                // csi seq
                let index_of_data = self.index_of_data;
                #[allow(clippy::comparison_chain)]
                if self.index_of_data < csi_item.start {
                    self.index_of_data = csi_item.start;
                    Some(Output::Text(&self.data[index_of_data..csi_item.start]))
                } else if self.index_of_data == csi_item.start {
                    self.index_of_data = csi_item.end;
                    Some(Output::Escape(csi_item.into()))
                } else {
                    // data已经遍历完成
                    if index_of_data >= self.data.len() {
                        return None;
                    }

                    self.index += 1;
                    // 已经是最后一个csi
                    if self.index >= csi_matches.len() {
                        return Some(Output::Text(&self.data[index_of_data..]));
                    }

                    let next = &csi_matches[self.index];
                    self.index_of_data = next.start;

                    Some(Output::Text(&self.data[index_of_data..next.start]))
                }
            } else {
                None
            };
        }

        None
    }
}

#[cfg(test)]
mod tests {
    #[cfg(all(not(feature = "std"), feature = "no_std"))]
    use alloc::vec;
    #[cfg(all(not(feature = "std"), feature = "no_std"))]
    use alloc::vec::Vec;

    #[cfg(feature = "std")]
    use std::vec::Vec;

    use super::*;
    use crate::enums::CSISequence::Color;

    #[test]
    fn test_iter() {
        let text = "\x1b[31mhello,world\x1b[m";
        let out: Vec<Output> = text.csi_parser().skip(1).collect();

        assert_eq!(
            out,
            vec![
                Output::Text("hello,world"),
                Output::Escape(Color(None, None, None))
            ]
        );
    }

    #[test]
    fn test_filter() {
        let text = "\x1b[31mhello,world\x1b[m";
        let out: Vec<Output> = text.csi_parser().filter(Output::is_text).collect();

        assert_eq!(out, vec![Output::Text("hello,world")]);
    }

    #[test]
    fn test_iter_1() {
        let text = "\x1b[31m";
        let out: Vec<Output> = text.csi_parser().collect();

        assert_eq!(out, vec![Output::Escape(Color(Some(31), None, None))]);
    }

    #[test]
    fn test_iter_2() {
        let text = "hello world";
        let out: Vec<Output> = text.csi_parser().collect();

        assert_eq!(out, vec![Output::Text("hello world")]);
    }

    #[test]
    fn test_iter_3() {
        let text = "";
        let out: Vec<Output> = text.csi_parser().collect();

        assert_eq!(out, vec![]);
    }

    #[test]
    fn test_iter_4() {
        let text = " ";
        let out: Vec<Output> = text.csi_parser().collect();

        assert_eq!(out, vec![Output::Text(" ")]);
    }

    #[test]
    fn test_iter_5() {
        let text = "hello world\x1b[31m";
        let out: Vec<Output> = text.csi_parser().collect();

        assert_eq!(
            out,
            vec![
                Output::Text("hello world"),
                Output::Escape(Color(Some(31), None, None))
            ]
        );
    }

    #[test]
    fn test_iter_6() {
        let text = "\x1b[mhello world\x1b[31m";
        let out: Vec<Output> = text.csi_parser().collect();

        assert_eq!(
            out,
            vec![
                Output::Escape(Color(None, None, None)),
                Output::Text("hello world"),
                Output::Escape(Color(Some(31), None, None))
            ]
        );
    }
}
