// SPDX-FileCopyrightText: 2015–2022 Felix A. Crux <felixc@felixcrux.com> and CONTRIBUTORS
// SPDX-License-Identifier: GPL-3.0-or-later

use rexiv2::Metadata;

/// 图像描述
pub const EXIF_TAG_IMAGE_DESCRIPTION: &str = "Exif.Image.ImageDescription";
/// 相机制造商
pub const EXIF_TAG_MAKE: &str = "Exif.Image.Make";
/// 相机型号
pub const EXIF_TAG_MODEL: &str = "Exif.Image.Model";
/// 图像方向
pub const EXIF_TAG_ORIENTATION: &str = "Exif.Image.Orientation";
/// 水平分辨率
pub const EXIF_TAG_X_RESOLUTION: &str = "Exif.Image.XResolution";
/// 垂直分辨率
pub const EXIF_TAG_Y_RESOLUTION: &str = "Exif.Image.YResolution";
/// 分辨率单位
pub const EXIF_TAG_RESOLUTION_UNIT: &str = "Exif.Image.ResolutionUnit";
/// 软件
pub const EXIF_TAG_SOFTWARE: &str = "Exif.Image.Software";
/// 文件修改日期和时间
pub const EXIF_TAG_DATE_TIME: &str = "Exif.Image.DateTime";
/// 艺术家
pub const EXIF_TAG_ARTIST: &str = "Exif.Image.Artist";
/// 版权信息
pub const EXIF_TAG_COPYRIGHT: &str = "Exif.Image.Copyright";
/// 原始拍摄时间
pub const EXIF_TAG_DATETIME_ORIGINAL: &str = "Exif.Photo.DateTimeOriginal";
/// 数字化时间
pub const EXIF_TAG_DATETIME_DIGITIZED: &str = "Exif.Photo.DateTimeDigitized";
/// 照片评论/注释。可以包含多行文本
pub const EXIF_TAG_USER_COMMENT: &str = "Exif.Photo.UserComment";
/// 曝光时间
pub const EXIF_TAG_EXPOSURE_TIME: &str = "Exif.Photo.ExposureTime";
/// GPS 日期戳
pub const EXIF_TAG_GPS_DATESTAMP: &str = "Exif.GPSInfo.GPSDateStamp";

fn main() {
    let file = "/home/jiang/rs/iv/data/1.jpg";
    rexiv2::initialize().expect("Unable to initialize rexiv2");

    if let Ok(meta) = Metadata::new_from_path(file) {
        let w = meta.get_pixel_width();
        let h = meta.get_pixel_height();
        println!("Size: {w}x{h}");

        if let Some(location) = meta.get_gps_info() {
            println!("Location: {location:?}");
        }

        if let Ok(time) = meta.get_tag_string("Exif.Image.DateTime") {
            println!("Time: {time:?}");
        }
        if meta
            .set_tag_string("Exif.Image.DateTime", "2008:11:01 21:15:07")
            .is_ok()
        {
            meta.save_to_file(file)
                .expect("Couldn't save metadata to file");
        }
    }
}
