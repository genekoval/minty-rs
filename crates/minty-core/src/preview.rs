mod audio;
mod image;
mod video;

use crate::obj::Bucket;

use fstore::Object;
use minty::Uuid;
use std::result;

pub type Result = result::Result<Option<Uuid>, String>;

pub struct Env {
    _image: image::Env,
}

impl Env {
    pub fn initialize() -> Self {
        Self {
            _image: image::Env::initialize(),
        }
    }
}

pub async fn generate_preview(bucket: &Bucket, object: &Object) -> Result {
    match object.r#type.as_str() {
        "audio" => audio::generate_preview(bucket, object).await,
        "image" => image::generate_preview(bucket, object).await,
        "video" => video::generate_preview(bucket, object).await,
        _ => Ok(None),
    }
}
