use tracing::warn;

#[derive(Debug)]
pub(crate) enum ImageFormat {
    Jpeg,
    Png,
}

impl ImageFormat {
    pub(crate) fn from_mime_or_extension(
        mime_type: Option<&str>,
        extension: Option<&str>,
    ) -> Option<Self> {
        if let Some(mime_type) = mime_type {
            match mime_type.to_lowercase().as_str() {
                "image/jpeg" => return Some(ImageFormat::Jpeg),
                "image/png" => return Some(ImageFormat::Png),
                _ => {
                    warn!("Unsupported mime-type: {}", mime_type);
                }
            }
        }
        if let Some(extension) = extension {
            if let Some((_, extension)) = extension.rsplit_once('.') {
                match extension.to_lowercase().as_str() {
                    "jpeg" => return Some(ImageFormat::Jpeg),
                    "png" => return Some(ImageFormat::Png),
                    _ => {
                        warn!("Unsupported extension: {}", extension);
                    }
                }
            }
        }
        None
    }
}
