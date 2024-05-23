mod utils;
use tokio::io::Error;

pub async fn open_rot_save() -> Result<(), Error> {
    let image_buf = utils::file_funcs::open_img("test_image.jpg");
    utils::file_funcs::save_img("rotated.png", image_buf.rotate90()).await?;
    Ok(())
}

#[tokio::test]
async fn it_works() {
    let _ = open_rot_save().await;
}

