#[derive(Clone)]
pub struct ConstImage<'a> {
    pub filename: &'a str,
    pub width: i32,
    pub height: i32,
}

impl ConstImage<'_> {
    pub fn build_image(&self) -> Image {
        Image {
            path: { format!("{}{}", "img/", self.filename.to_owned()).into() },
            width: self.width.to_string(),
            height: self.height.to_string(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Image {
    pub path: String,
    pub width: String,
    pub height: String,
}
