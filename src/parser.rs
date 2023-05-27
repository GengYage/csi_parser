#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;
use crate::{CSI, SEPARATOR};

/// A match.
#[derive(Debug, PartialEq, Eq)]
pub struct Match<'t> {
    /// First byte index.
    pub start: usize,
    /// Last byte index + 1.
    pub end: usize,
    /// The text slice (ie `text[start..end]`).
    /// Note that the range is `(start..end]`.
    pub csi_text: &'t str,
    pub original_text: &'t str,
    /// The CSI type (ie `m`, `A` `..`)
    pub csi_type: u8,
}

impl Match<'_> {
    /// full define is `pub fn parse_ansi_seq<'a>(&'a self) -> Vec<Output<'a>>`
    /// ```
    /// use csi_parser::parser::parse;
    ///
    /// let ansi_text = "Hello, \x1b[;31;4mworld\x1b[0m\x1b[10;20Hm!\x1b[m\x1b?25h";
    /// let parsed: Vec<_> = parse(ansi_text);
    /// for result in parsed {
    ///    println!("{:?}", result.parse_csi());
    /// }
    ///
    /// // output is:
    /// // ["", "31", "4"]
    /// // ["0"]
    /// // ["10", "20"]
    /// // [""]
    /// // ["?25"]
    /// ```
    pub fn parse_csi(&self) -> Vec<&str> {
        // 所有的参数
        let mut params: Vec<&str> = self.csi_text[CSI.len()..].split(SEPARATOR).collect();

        // 有参数,则证明存在`;`,是多个参数的情况
        if !params.is_empty() {
            let last = params.pop();
            unsafe {
                let last_param = last.unwrap_unchecked();
                params.push(&last_param[0..(last_param.len() - 1)]);
            }
            return params;
        }

        // 否则是单个参数
        if self.csi_text.len() > CSI.len() + 1 {
            params.push(&self.csi_text[CSI.len()..(self.end - 1)]);
        } else {
            // 不存在参数,用0填充
            params.push("0");
        }

        params
    }
}

/// Parses CSI escape codes from the given text, returning a vector of `Match`.
///
/// ```rust
/// use csi_parser::parser::parse;
/// let ansi_text = "Hello, \x1b[31;4mworld\x1b[0m!";
/// let parsed: Vec<_> = parse(ansi_text)
///     .into_iter()
///     .map(|m| (m.start, m.end))
///     .collect();
/// assert_eq!(
///     parsed,
///     vec![(7, 14), (19, 23)],
/// );
/// ```
pub fn parse(text: &str) -> Vec<Match> {
    let mut v = Vec::with_capacity(8);
    let csi_len = CSI.len();

    let mut s = text;
    let mut start = 0;
    let mut end = start + csi_len;

    while end <= text.len() {
        // start of a CSI seq
        if s.starts_with(CSI) {
            let mut byte = text.as_bytes()[end];

            // get end or CSI seq
            while end < text.len()
                // 必须是参数字节或者是中间字节
                && (crate::parameter_byte(byte) || (crate::intermediate_byte(byte))) {
                // 更新end
                end += 1;
                // 更新byte
                byte = text.as_bytes()[end];
            }

            // 紧跟着的不是终结字符
            byte = text.as_bytes()[end];
            if !crate::terminated_byte(byte) {
                break;
            }

            // 越界检查,获取CSI的切片
            let end = end + 1;
            if end > text.len() {
                break;
            }

            v.push(Match {
                start,
                end,
                csi_text: &text[start..end],
                original_text: &text[end..text.len()],
                csi_type: unsafe { (text[(end - 1)..end]).chars().next().unwrap_unchecked() as u8 },
            });

            start = end;
        } else {
            start += s.chars().next().expect("non-empty-str").len_utf8();
        }

        s = &text[start..];
        end = start + csi_len;
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let ansi_text = "Hello, \x1b[31;4mworld\x1b[0m!";
        let parsed = parse(ansi_text);
        assert_eq!(
            parsed,
            vec![
                Match {
                    start: 7,
                    end: 14,
                    csi_text: "\x1b[31;4m",
                    original_text: "world\x1b[0m!",
                    csi_type: b'm',
                },
                Match {
                    start: 19,
                    end: 23,
                    csi_text: "\x1b[0m",
                    original_text: "!",
                    csi_type: b'm',
                },
            ]
        );
    }

    #[test]
    fn parse_string_with_different_chars() {
        let t = "👋, \x1b[31;4m🌍\x1b[0m!";
        let parsed = parse(t);
        assert_eq!(
            parsed,
            vec![
                Match {
                    start: 6,
                    end: 13,
                    csi_text: "\x1b[31;4m",
                    original_text: "🌍\x1b[0m!",
                    csi_type: b'm',
                },
                Match {
                    start: 17,
                    end: 21,
                    csi_text: "\x1b[0m",
                    original_text: "!",
                    csi_type: b'm',
                },
            ]
        );
    }

    #[test]
    fn parse_string_with_set_cursor_ansi() {
        let t = "\x1b[31Ahello!";
        let parsed = parse(t);
        assert_eq!(
            parsed,
            vec![
                Match {
                    start: 0,
                    end: 5,
                    csi_text: "\x1b[31A",
                    original_text: "hello!",
                    csi_type: b'A',
                },
            ]
        );
    }

    #[test]
    fn malformed_escape() {
        let x = parse("oops\x1b[\n");

        assert_eq!(x, vec![]);
    }

    #[test]
    fn reset_color() {
        let x = parse("oops\x1b[0m");

        assert_eq!(x, vec![
            Match {
                start: 4,
                end: 8,
                csi_text: "\x1b[0m",
                original_text: "",
                csi_type: b'm',
            },
        ]);
    }
}