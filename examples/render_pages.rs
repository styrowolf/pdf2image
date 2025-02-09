use pdf2image::{PDF2ImageError, RenderOptionsBuilder, PDF};

fn main() -> Result<(), PDF2ImageError> {
    let pdf = PDF::from_file("examples/pdfs/ropes.pdf").unwrap();
    let pages = pdf.render(
        pdf2image::Pages::Range(1..=8),
        RenderOptionsBuilder::default().pdftocairo(true).build()?,
    )?;

    std::fs::create_dir("examples/out").unwrap();
    for (i, page) in pages.iter().enumerate() {
        page.save_with_format(format!("examples/out/{}.jpg", i + 1), image::ImageFormat::Jpeg)?;
    }

    Ok(())
}
