extern crate opencv;

use opencv::{
    prelude::*,
    videoio::{self, VideoCaptureTrait},
    imgcodecs,
};

pub fn main1() -> opencv::Result<()> {

    let mut cam = videoio::VideoCapture::default()?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    for i in 0..10 {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        let size = frame.size()?;
        print!("size: {:?}", &size);

        let file_name = format!("frame_{}.jpg", i);
        imgcodecs::imwrite(&file_name, &frame, &Default::default())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
