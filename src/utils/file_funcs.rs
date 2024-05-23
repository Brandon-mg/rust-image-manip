use std::path::Path;
use image::DynamicImage;
use tokio::io::Error;

pub async fn save_img(img_name: &str, img_buf: DynamicImage) -> Result<(), Error>{
    img_buf.save(Path::new(img_name)).unwrap();
    Ok(())
}

pub fn open_img(path: &str) -> DynamicImage{
    return match image::open(Path::new(path)) {
        Ok(buf) => buf,
        Err(_e) => DynamicImage::ImageRgba16(image::ImageBuffer::new(100, 100))
    };


}