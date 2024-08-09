use super::icon;

use maud::{html, Markup, Render};

#[derive(Debug)]
pub struct ObjectPreview {
    object: minty::ObjectPreview,
    rounded_corners: bool,
}

impl ObjectPreview {
    pub fn new(object: minty::ObjectPreview) -> Self {
        Self {
            object,
            rounded_corners: false,
        }
    }

    pub fn rounded_corners(mut self) -> Self {
        self.rounded_corners = true;
        self
    }

    fn preview(&self) -> Option<String> {
        self.object
            .preview_id
            .map(|id| format!("/object/{id}/data"))
    }

    fn file_type(&self) -> String {
        self.object.r#type.to_uppercase()
    }
}

impl From<minty::ObjectPreview> for ObjectPreview {
    fn from(value: minty::ObjectPreview) -> Self {
        Self::new(value)
    }
}

impl Render for ObjectPreview {
    fn render(&self) -> Markup {
        html! {
            @if let Some(preview) = self.preview() {
                img src=(preview)
                    .max-width-full
                    .rounded-corners[self.rounded_corners];
            } @else {
                .flex-row .center {
                    .object-placeholder { (icon::FILE_FILL) }
                    .object-placeholder-text
                    .absolute
                    .background-color
                    .bold
                    .z1 {
                        (self.file_type())
                    }
                }
            }
        }
    }
}
