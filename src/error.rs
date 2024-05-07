use thiserror::Error;

/// A `Result` type alias using `PDF2ImgError` instances as the error variant.
pub type Result<T> = std::result::Result<T, PDF2ImageError>;

/// pdf2img error variants
#[derive(Error, Debug)]
pub enum PDF2ImageError {
    /// An I/O error.
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    /// A UTF-8 parsing error.
    #[error("utf-8 parsing error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    /// An integer parsing error.
    #[error("int parsing error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    /// An image error.
    #[error("image error: {0}")]
    ImageError(#[from] image::ImageError),
    /// An error indicating that the builder is misconfigured.
    #[error("RenderOptionsBuilder error: {0}")]
    RenderOptionsBuilder(#[from] crate::render_options::RenderOptionsBuilderError),
    /// An error indicating that the PDF is encrypted and no password was provided.
    #[error("No password given for encrypted PDF")]
    NoPasswordForEncryptedPDF,
}
