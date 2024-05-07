use image::DynamicImage;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use crate::error::{PDF2ImageError, Result};
use crate::render_options::RenderOptions;
use crate::utils::{extract_pdf_info, get_executable_path};

/// A PDF file
///
/// This struct wraps the bytes of an PDF and additional information about the PDF, such as the number of pages and whether the PDF is encrypted.
///
/// # Usage
///
/// ```
/// use pdf2image::{PDF, Pages, RenderOptionsBuilder};
///
/// fn main() -> Result<(), pdf2img::Error> {
///     let pdf = PDF::from_file("examples/pdfs/ropes.pdf")?;
///     let rendered_pages = pdf.render(Pages::All, RenderOptionsBuilder::default().build()?)?;
/// }
/// ```
///
/// # Rationale
/// Storing the page count prevents calls to `pdfinfo` for every call to `render()`.
pub struct PDF {
    data: Vec<u8>,
    page_count: u32,
    encrypted: bool,
}

impl PDF {
    /// Constructs a PDF from bytes.
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        let (page_count, encrypted) = extract_pdf_info(&data)?;
        Ok(Self {
            data,
            page_count,
            encrypted,
        })
    }

    /// Constructs a PDF from a PDF file.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let data = std::fs::read(path)?;
        let (page_count, encrypted) = extract_pdf_info(&data)?;
        Ok(Self {
            data,
            page_count,
            encrypted,
        })
    }

    /// Returns the number of pages in the PDF.
    pub fn page_count(&self) -> u32 {
        self.page_count
    }

    /// Returns whether the PDF is encrypted.
    pub fn is_encrypted(&self) -> bool {
        self.encrypted
    }

    /// Renders the PDF to images.
    pub fn render(
        &self,
        pages: Pages,
        options: impl Into<Option<RenderOptions>>,
    ) -> Result<Vec<image::DynamicImage>> {
        let pages_range: Vec<_> = match pages {
            Pages::Range(range) => range
                .filter(|page| {
                    if *page > self.page_count {
                        //eprintln!("Page {} does not exist in the PDF.", page);
                        false
                    } else if *page < 1 {
                        //eprintln!("Page {} does not exist in the PDF.", page);
                        false
                    } else {
                        true
                    }
                })
                .collect(),
            Pages::All => (0..=self.page_count).collect(),
            Pages::Single(page) => (page..page + 1).collect(),
        };

        let options = options.into().unwrap_or_default();

        if self.encrypted && options.password.is_none() {
            return Err(PDF2ImageError::NoPasswordForEncryptedPDF);
        }

        let cli_options = options.to_cli_args();

        let executable = get_executable_path(if options.pdftocairo {
            "pdftocairo"
        } else {
            "pdftoppm"
        });

        let poppler_args = if options.pdftocairo {
            vec![
                "-".to_string(),
                "-".to_string(),
                "-jpeg".to_string(),
                "-singlefile".to_string(),
            ]
        } else {
            vec!["-jpeg".to_string(), "-singlefile".to_string()]
        };

        let images_results: Vec<Result<DynamicImage>> = pages_range
            .par_iter()
            .map(|page| {
                let args = [
                    poppler_args.clone(),
                    vec![
                        "-f".to_string(),
                        format!("{page}"),
                        "-l".to_string(),
                        format!("{page}"),
                    ],
                    cli_options.clone(),
                ]
                .concat();

                let mut child = Command::new(&executable)
                    .args(&args)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("failed to execute process");

                // UNWRAP SAFETY: The child process is guaranteed to have a stdin as .stdin(Stdio::piped()) was called
                child.stdin.as_mut().unwrap().write_all(&self.data)?;

                let output = child.wait_with_output()?;
                let image =
                    image::load_from_memory_with_format(&output.stdout, image::ImageFormat::Jpeg)?;

                Ok(image)
            })
            .collect();

        let mut images = Vec::with_capacity(images_results.len());

        for image in images_results {
            match image {
                Ok(image) => images.push(image),
                Err(e) => return Err(e),
            }
        }

        Ok(images)
    }
}

#[derive(Debug, Clone)]
/// Specifies which pages to render
pub enum Pages {
    All,
    Range(std::ops::RangeInclusive<u32>),
    Single(u32),
}
