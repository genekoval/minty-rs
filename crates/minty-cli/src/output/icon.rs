use std::fmt::{self, Display, Write};

#[derive(Clone, Copy, Debug)]
pub struct Icon(char);

impl Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(self.0)
    }
}

/// nf-md-account 󰀄
pub const ACCOUNT: Icon = Icon('\u{f0004}');

/// nf-oct-file_binary 
pub const BINARY: Icon = Icon('\u{f471}');

/// nf-md-calendar 󰃭
pub const CALENDAR: Icon = Icon('\u{f00ed}');

/// nf-md-clock 󰥔
pub const CLOCK: Icon = Icon('\u{f0954}');

/// nf-md-comment 󰅺
pub const COMMENT: Icon = Icon('\u{f017a}');

/// nf-md-file_document 󰈙
pub const DOCUMENT: Icon = Icon('\u{f0219}');

/// nf-md-email 󰇮
pub const EMAIL: Icon = Icon('\u{f01ee}');

/// nf-md-eye 󰈈
pub const EYE: Icon = Icon('\u{f0208}');

/// nf-md-harddisk 󰋊
pub const HARDDISK: Icon = Icon('\u{f02ca}');

/// nf-md-file_image 󰈟
pub const IMAGE: Icon = Icon('\u{f021f}');

/// nf-md-pencil 󰏫
pub const PENCIL: Icon = Icon('\u{f03eb}');

/// nf-md-pound 󰐣
pub const POUND: Icon = Icon('\u{f0423}');

/// nf-md-seal 󰑺
pub const SEAL: Icon = Icon('\u{f047a}');

/// nf-md-tag 󰓹
pub const TAG: Icon = Icon('\u{f04f9}');

/// nf-oct-trash 
pub const TRASH: Icon = Icon('\u{f48e}');
