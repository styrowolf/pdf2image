#[derive(derive_builder::Builder)]
/// Options for rendering PDFs
pub struct RenderOptions {
    #[builder(default = "DPI::Uniform(150)")]
    /** Resolution in dots per inch */
    pub resolution: DPI,
    #[builder(setter(into, strip_option), default)]
    /** Scale pages to a certain number of pixels */
    pub scale: Option<Scale>,
    #[builder(default)]
    /** Render pages in grayscale */
    pub greyscale: bool,
    #[builder(setter(into, strip_option), default)]
    /** Crop a specific section of the page */
    pub crop: Option<Crop>,
    #[builder(setter(into, strip_option), default)]
    /** Password to unlock encrypted PDFs */
    pub password: Option<Password>,
    /** Use pdftocairo instead of pdftoppm */
    #[builder(default)]
    pub pdftocairo: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            resolution: DPI::Uniform(150),
            scale: None,
            greyscale: false,
            crop: None,
            password: None,
            pdftocairo: false,
        }
    }
}

impl RenderOptions {
    pub fn to_cli_args(&self) -> Vec<String> {
        let mut args = vec![];

        match self.resolution {
            DPI::Uniform(dpi) => {
                args.push("-r".to_string());
                args.push(dpi.to_string());
            }
            DPI::XY(dpi_x, dpi_y) => {
                args.push("-rx".to_string());
                args.push(dpi_x.to_string());
                args.push("-ry".to_string());
                args.push(dpi_y.to_string());
            }
        }

        if let Some(scale) = &self.scale {
            match scale {
                Scale::Uniform(scale) => {
                    args.push("-scale-to".to_string());
                    args.push(scale.to_string());
                }
                Scale::X(scale_x) => {
                    args.push("-scale-to-x".to_string());
                    args.push(scale_x.to_string());
                }
                Scale::Y(scale_y) => {
                    args.push("-scale-to-y".to_string());
                    args.push(scale_y.to_string());
                }
                Scale::XY(scale_x, scale_y) => {
                    args.push("-scale-to-x".to_string());
                    args.push(scale_x.to_string());
                    args.push("-scale-to-y".to_string());
                    args.push(scale_y.to_string());
                }
            }
        }

        if self.greyscale {
            args.push("-gray".to_string());
        }

        if let Some(crop) = &self.crop {
            args.push("-cropbox".to_string());
            let (x, y) = (crop.inner.x, crop.inner.y);
            let (width, height) = (crop.inner.width, crop.inner.height);
            args.push("-x".to_string());
            args.push(x.to_string());
            args.push("-y".to_string());
            args.push(y.to_string());
            args.push("-W".to_string());
            args.push(width.to_string());
            args.push("-H".to_string());
            args.push(height.to_string());
        }

        if let Some(password) = &self.password {
            match password {
                Password::User(password) => {
                    args.push("-upw".to_string());
                    args.push(password.clone());
                }
                Password::Owner(password) => {
                    args.push("-opw".to_string());
                    args.push(password.clone());
                }
            }
        }

        args
    }
}

#[derive(Debug, Clone)]
/** Password to unlock encrypted PDFs */
pub enum Password {
    User(String),
    Owner(String),
}

#[derive(Debug, Clone)]
/** Specifies resolution in terms of dots per inch */
pub enum DPI {
    /** DPI for both axes */
    Uniform(u32),
    /** DPI for x and y axis */
    XY(u32, u32),
}

#[derive(Debug, Clone)]
/** Scales pages to a certain number of pixels */
pub enum Scale {
    /** scales each page to fit within scale-to*scale-to pixel box */
    Uniform(u32),
    /** scales each page horizontally to fit in scale-to-x pixels */
    X(u32),
    /** scales each page vertically to fit in scale-to-y pixels */
    Y(u32),
    /** scales each page to fit within scale-to-x*scale-to-y pixel box */
    XY(u32, u32),
}

#[derive(Debug, Clone)]
/// Crop a specific section of the page
pub struct Crop {
    inner: image::math::Rect,
}

impl Crop {
    pub fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        let (min_x, max_x) = match x1 < x2 {
            true => (x1, x2),
            false => (x2, x1),
        };

        let (min_y, max_y) = match y1 < y2 {
            true => (y1, y2),
            false => (y2, y1),
        };

        Self {
            inner: image::math::Rect {
                x: min_x,
                y: min_y,
                width: max_x - min_x,
                height: max_y - min_y,
            },
        }
    }

    pub fn from_top_left(width: u32, height: u32, top_left: (u32, u32)) -> Self {
        Self {
            inner: image::math::Rect {
                x: top_left.0,
                y: top_left.1,
                width,
                height,
            },
        }
    }

    pub fn square(size: u32, top_left: (u32, u32)) -> Self {
        Self {
            inner: image::math::Rect {
                x: top_left.0,
                y: top_left.1,
                width: size,
                height: size,
            },
        }
    }
}
