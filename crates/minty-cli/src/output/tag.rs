use super::{color, icon, metadata::Metadata, time::FormatDate, HumanReadable};

use minty::{Tag, TagPreview};
use owo_colors::OwoColorize;
use std::io::{Result, Write};

impl HumanReadable for Tag {
    fn human_readable<W: Write>(&self, w: &mut W, indent: usize) -> Result<()> {
        self.profile.human_readable(w, indent)?;

        Metadata::new()
            .row("ID", icon::POUND, self.id)
            .row("Posts", icon::DOCUMENT, self.post_count)
            .row("Created", icon::CALENDAR, self.profile.created.long_date())
            .print(indent, w)
    }
}

impl HumanReadable for TagPreview {
    fn human_readable<W: Write>(&self, w: &mut W, indent: usize) -> Result<()> {
        writeln!(w, "{}", self.name.bold())?;

        write!(w, "{:1$}", "", indent)?;
        writeln!(
            w,
            "{} {}",
            icon::POUND.fg::<color::Label>(),
            self.id.fg::<color::Secodary>()
        )
    }
}
