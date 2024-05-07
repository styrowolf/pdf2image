use pdf2image::{PDF2ImageError, RenderOptionsBuilder, PDF};

fn main() -> Result<(), PDF2ImageError> {
    let pdf = PDF::from_file("examples/pdfs/ropes.pdf").unwrap();
    let pages = pdf.render(
        pdf2image::Pages::Range(1..=8),
        RenderOptionsBuilder::default().pdftocairo(true).build()?,
    );
    println!("{:?}", pages.unwrap().len());

    Ok(())
}
