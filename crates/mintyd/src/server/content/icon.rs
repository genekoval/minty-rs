use maud::{html, Markup, PreEscaped, Render};

#[derive(Clone, Copy, Debug)]
pub struct Icon(&'static str);

impl Icon {
    pub fn inline(self) -> Markup {
        html! {
            span .inline-icon { (self) }
        }
    }

    fn svg(&self) -> &str {
        // Remove any trailing newlines from SVG file contents
        self.0.trim_end()
    }
}

impl Render for Icon {
    fn render(&self) -> Markup {
        html! {
            (PreEscaped(self.svg()))
        }
    }
}

macro_rules! icon {
    ($name:literal) => {
        Icon(include_str!(concat!("icon/", $name, ".svg")))
    };
}

pub const ALIGN_LEFT: Icon = icon!("align_left");
pub const BADGE_CHECK: Icon = icon!("badge_check");
pub const CALENDAR: Icon = icon!("calendar");
pub const CLOCK: Icon = icon!("clock");
pub const COMMENT: Icon = icon!("comment");
pub const DOTS_6_ROTATE: Icon = icon!("6-dots-rotate");
pub const ENVELOPE: Icon = icon!("envelope");
pub const FILE: Icon = icon!("file");
pub const FILE_FILL: Icon = icon!("file_fill");
pub const FILE_IMAGE: Icon = icon!("file_image");
pub const HASH: Icon = icon!("hash");
pub const HOME: Icon = icon!("home");
pub const LINK: Icon = icon!("link");
pub const PENCIL: Icon = icon!("pencil");
pub const USER_CIRCLE: Icon = icon!("user_circle");
