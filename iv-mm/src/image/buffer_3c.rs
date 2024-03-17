use std::ffi::c_void;

use image::RgbImage;
use opencv::core::CV_8UC3;
use opencv::prelude::*;

/* 三通道缓冲区

Buffer3C 是一个三通道缓冲区，用于图像数据的传输, 格式转化的中间表示.
注意: 不能图通过临时的Mat实现 ToInputArray 和 ToOutputArray!!!
缺陷: 还不支持对齐
 */
pub struct Buffer3C {
    /// 数据缓冲区
    pub data: Vec<u8>,
    /// 数据行数(Height)
    pub rows: i32,
    /// 数据列数(Width)
    pub cols: i32,
    /// 数据类型编号
    pub elem_type: i32,
}

impl Buffer3C {
    /// 将缓冲区作为 Mat 访问
    pub fn as_mat(&self) -> opencv::Result<Mat> {
        unsafe {
            let data_ptr = self.data.as_ptr() as *mut c_void;
            Mat::new_rows_cols_with_data_def(self.rows, self.cols, self.elem_type, data_ptr)
        }
    }
}

impl From<RgbImage> for Buffer3C {
    /// 从 RgbImage 构建 Buffer3C
    fn from(image: RgbImage) -> Self {
        let (width, height) = image.dimensions();
        Self {
            data: image.into_raw(),
            rows: height as i32,
            cols: width as i32,
            elem_type: CV_8UC3,
        }
    }
}

/*
impl ToInputArray for Buffer3C {
    /// 作为 OpenCV 函数输入参数访问
    fn input_array(&self) -> opencv::Result<_InputArray> {
        let mat = self.as_mat()?;
        mat.input_array() // FIXME: mat被销毁后 _InputArray 指针悬空
    }
}

impl ToOutputArray for Buffer3C {
    /// 作为 OpenCV 函数输出参数访问
    fn output_array(&mut self) -> opencv::Result<_OutputArray> {
        let mut mat = self.as_mat()?;
        mat.output_array()
    }
}
*/

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use opencv::core::{MatTrait, Scalar};
    use opencv::highgui;

    use crate::image::{load_image, Buffer3C};

    #[test]
    fn it_works() {
        let p = PathBuf::from("/home/jiang/rs/iv/iv-mm/data/lena.jpg");
        let im = load_image(&p).unwrap();
        let im = im.to_rgb8();

        let buffer = Buffer3C::from(im);
        let mut mat = buffer.as_mat().unwrap();

        let window = "video capture";
        highgui::named_window(window, highgui::WINDOW_AUTOSIZE).unwrap();
        highgui::imshow(window, &mat).unwrap();
        let _key = highgui::wait_key(0).unwrap();
        mat.set_scalar(Scalar::all(0.0)).unwrap();

        let mat1 = buffer.as_mat().unwrap();
        highgui::imshow(window, &mat1).unwrap();
        let _key = highgui::wait_key(0).unwrap();
    }
}
