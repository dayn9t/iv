use opencv::core::Mat;
use opencv::highgui;
use opencv::imgcodecs;
use opencv::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the image
    let image_path = "/home/jiang/rs/iv/assets/images/lena.jpg";
    let img = imgcodecs::imread(image_path, imgcodecs::IMREAD_COLOR)?;

    // Create two windows
    let window1 = "Window 1";
    let window2 = "Window 2";
    let window3 = "Window 3";
    let window4 = "Window 4";
    highgui::named_window(window1, highgui::WINDOW_GUI_NORMAL)?;
    highgui::named_window(window2, highgui::WINDOW_GUI_NORMAL)?;
    highgui::named_window(window3, highgui::WINDOW_GUI_NORMAL)?;
    highgui::named_window(window4, highgui::WINDOW_GUI_NORMAL)?;

    //highgui::resize_window(window1, 320, 240)?;
    //highgui::resize_window(window2, 320, 240)?;
    //highgui::resize_window(window3, 320, 240)?;
    //highgui::resize_window(window4, 320, 240)?;

    highgui::move_window(window1, 100, 100)?;
    highgui::move_window(window2, 1200, 100)?;

    // Display the image in both windows
    highgui::imshow(window1, &img)?;
    highgui::imshow(window2, &img)?;
    highgui::imshow(window3, &img)?;
    highgui::imshow(window4, &img)?;

    // Wait for a key press
    highgui::wait_key(0)?;

    Ok(())
}
