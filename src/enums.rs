use core::fmt::{Display, Formatter, Result as DisplayResult};

use crate::parser::Match;

/// A subset of CSI escape sequences. maybe add more.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CSISequence {
    Escape,
    Color(Option<usize>, Option<usize>, Option<usize>),
    CursorPos(Option<usize>, Option<usize>),
    CursorUp(Option<usize>),
    CursorDown(Option<usize>),
    CursorForward(Option<usize>),
    CursorBackward(Option<usize>),
    CursorSave,
    CursorRestore,
    EraseDisplay(ClearMode),
    EraseLine(ClearMode),
    EnableAttr(Attr),
    ResetAttr(Attr),
}

impl From<&Match<'_>> for CSISequence {
    fn from(match_data: &Match<'_>) -> Self {
        use CSISequence::*;
        let params = match_data.parse_csi();
        match match_data.csi_type {
            // 设置文本属性
            b'm' => {
                let foreground_color = str_to_usize(params.first());
                // 兼容两个参数和一个参数的情况
                let background_color = str_to_usize({
                    // 参数数量大于2,才会有背景色参数
                    if params.len() > 2 {
                        params.get(1)
                    } else {
                        None
                    }
                });
                let style = str_to_usize(if params.len() > 1 {
                    // 参数数量大于1,必然有属性参数
                    params.last()
                } else {
                    None
                });
                Color(foreground_color, background_color, style)
            }

            // 设置光标位置
            b'H' => {
                let row = str_to_usize(params.first());
                let col = str_to_usize(params.last());
                CursorPos(row, col)
            }

            // 光标上移
            b'A' => {
                let row = str_to_usize(params.first());
                CursorUp(row)
            }

            // 光标下移
            b'B' => {
                let row = str_to_usize(params.first());
                CursorDown(row)
            }

            // 光标右移
            b'C' => {
                let row = str_to_usize(params.first());
                CursorForward(row)
            }

            // 光标左移
            b'D' => {
                let row = str_to_usize(params.first());
                CursorBackward(row)
            }

            // 保存光标
            b's' => CursorSave,

            // 恢复光标
            b'u' => CursorRestore,

            // 清除屏幕
            b'J' => {
                let param = str_to_usize(params.first());
                EraseDisplay(ClearMode::from(param))
            }

            // 清除行
            b'K' => {
                let param = str_to_usize(params.first());
                EraseLine(ClearMode::from(param))
            }

            // 启用属性
            b'h' => {
                let param = str_to_usize(params.first());
                EnableAttr(Attr::from(param))
            }

            // 关闭属性
            b'l' => {
                let param = str_to_usize(params.first());
                ResetAttr(Attr::from(param))
            }

            // 未定义
            _ => Escape,
        }
    }
}

#[inline]
/// CSI param to usize
pub(crate) fn str_to_usize(num_str: Option<&&str>) -> Option<usize> {
    match num_str {
        None => None,
        Some(str) => {
            if str.is_empty() {
                return None;
            }

            // 兼容`?`参数
            if str.starts_with('?') {
                return str_to_usize(Some(&&str[1..str.len()]));
            }

            // 其他情况暂不处理
            // todo trace
            str.parse::<usize>().ok()
        }
    }
}

impl Display for CSISequence {
    /// 不打印0x1B,避免打印被转义
    fn fmt(&self, formatter: &mut Formatter) -> DisplayResult {
        use CSISequence::*;
        match self {
            Escape => write!(formatter, "\u{1b}"),
            Color(strong, color, state) => {
                write!(formatter, "[{:?};{:?};{:?}m", strong, color, state)
            }
            CursorPos(row, col) => write!(formatter, "[{:?};{:?}H", row, col),
            CursorUp(amt) => write!(formatter, "[{:?}A", amt),
            CursorDown(amt) => write!(formatter, "[{:?}B", amt),
            CursorForward(amt) => write!(formatter, "[{:?}C", amt),
            CursorBackward(amt) => write!(formatter, "[{:?}D", amt),
            CursorSave => write!(formatter, "[s"),
            CursorRestore => write!(formatter, "[u"),
            EraseDisplay(mode) => write!(formatter, "[{}J", mode),
            EraseLine(mode) => write!(formatter, "[{}K", mode),
            EnableAttr(attr) => write!(formatter, "[?{}h", attr),
            ResetAttr(attr) => write!(formatter, "[?{}l", attr),
        }
    }
}

/// CSI `h` mode
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Attr {
    None = 0,
    Cursor = 25,
    AutoWrap = 7,
}

impl From<Option<usize>> for Attr {
    fn from(value: Option<usize>) -> Self {
        match value {
            None => Attr::None,
            Some(mode) => match mode {
                0 => Attr::None,
                25 => Attr::Cursor,
                7 => Attr::AutoWrap,
                _ => Attr::None,
            },
        }
    }
}

impl Display for Attr {
    fn fmt(&self, formatter: &mut Formatter) -> DisplayResult {
        match self {
            Attr::None => write!(formatter, "{}", 0),
            Attr::Cursor => write!(formatter, "{}", 25),
            Attr::AutoWrap => write!(formatter, "{}", 7),
        }
    }
}

/// CSI `J`,`K` mode
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ClearMode {
    After = 0,
    Before = 1,
    All = 2,
}

impl From<Option<usize>> for ClearMode {
    fn from(value: Option<usize>) -> Self {
        match value {
            None => ClearMode::After,
            Some(mode) => match mode {
                0 => ClearMode::After,
                1 => ClearMode::Before,
                2 => ClearMode::All,
                _ => ClearMode::After,
            },
        }
    }
}

impl Display for ClearMode {
    fn fmt(&self, formatter: &mut Formatter) -> DisplayResult {
        match self {
            ClearMode::After => write!(formatter, "{}", 0),
            ClearMode::Before => write!(formatter, "{}", 1),
            ClearMode::All => write!(formatter, "{}", 2),
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(all(not(feature = "std"), feature = "no_std"))]
    use alloc::vec;

    use super::*;
    use crate::enums::CSISequence::*;
    use crate::parser::parse;

    #[test]
    fn parse_test() {
        let ansi_text = "Hello, \x1b[;;4mworld\x1b[0K!\x1b[?7h";

        let mut csi_seqs = vec![];
        for x in parse(ansi_text) {
            let csi_seq: CSISequence = (&x).into();
            csi_seqs.push(csi_seq);
        }

        assert_eq!(
            csi_seqs,
            vec![
                Color(None, None, Some(4)),
                EraseLine(ClearMode::After),
                EnableAttr(Attr::AutoWrap),
            ]
        );
    }

    #[test]
    fn parse_string_with_different_chars() {
        let t = "👋, \x1b[31;4m🌍\x1b[0m!";

        let mut csi_seqs = vec![];
        for x in parse(t) {
            let csi_seq: CSISequence = (&x).into();
            csi_seqs.push(csi_seq);
        }

        assert_eq!(
            csi_seqs,
            vec![Color(Some(31), None, Some(4)), Color(Some(0), None, None),]
        );
    }

    #[test]
    fn parse_string_with_set_cursor_ansi() {
        let t = "\x1b[31Ahello!";
        let mut csi_seqs = vec![];
        for x in parse(t) {
            let csi_seq: CSISequence = (&x).into();
            csi_seqs.push(csi_seq);
        }

        assert_eq!(csi_seqs, vec![CursorUp(Some(31)),]);
    }

    #[test]
    fn malformed_escape() {
        let mut csi_seqs = vec![];
        for x in parse("oops\x1b[\n") {
            let csi_seq: CSISequence = (&x).into();
            csi_seqs.push(csi_seq);
        }

        assert_eq!(csi_seqs, vec![]);
    }

    #[test]
    fn reset_color() {
        let x = parse("oops\x1b[0m");

        let mut csi_seqs = vec![];
        for m in x {
            let csi_seq: CSISequence = (&m).into();
            csi_seqs.push(csi_seq);
        }

        assert_eq!(csi_seqs, vec![Color(Some(0), None, None),]);
    }
}
