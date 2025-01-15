use rexiv2::Metadata;
use std::path::Path;

pub use chrono::NaiveDateTime;
pub use rexiv2::GpsInfo;

use iv_core::geo::Size;
use rx_core::text::AnyResult;

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

pub struct ExifManager {
    meta: Metadata,
}

impl ExifManager {
    /// 创建一个新的 ExifManager 实例
    pub fn new(file_path: impl AsRef<Path>) -> AnyResult<Self> {
        rexiv2::initialize()?;
        let metadata = Metadata::new_from_path(file_path.as_ref())?;
        Ok(Self { meta: metadata })
    }

    /// 保存图片的注释信息
    pub fn save_comment(file_path: impl AsRef<Path>, comment: &str) -> AnyResult<()> {
        let mut exif = Self::new(file_path.as_ref())?;
        exif.set_comment(comment)?;
        exif.save(file_path.as_ref())?;
        Ok(())
    }

    /// 注入EXIF信息
    pub fn inject_exif(file: &Path, time: NaiveDateTime, gps: GpsInfo, comment: Option<&str>) {
        let mut exif = ExifManager::new(file).unwrap();
        exif.set_gps_coordinates(gps).unwrap();
        exif.set_time(time).unwrap();
        if let Some(comment) = comment {
            exif.set_comment(comment).unwrap();
        }
        exif.save(file).unwrap();

        let p = exif.get_gps_coordinates().unwrap();
        println!("gps: {:?}", p);
    }

    /// 复制EXIF信息
    pub fn copy_exif(src: &Path, dst: &Path) {
        let exif = ExifManager::new(src).unwrap();
        exif.save(dst).unwrap();
    }

    /// 获取图片的尺寸
    pub fn get_size(&self) -> Size {
        let width = self.meta.get_pixel_width();
        let height = self.meta.get_pixel_height();
        Size { width, height }
    }

    /// 获取图片的修改时间
    pub fn get_time(&self) -> Option<NaiveDateTime> {
        self.meta
            .get_tag_string(EXIF_TAG_DATE_TIME)
            .ok()
            .and_then(|time_str| NaiveDateTime::parse_from_str(&time_str, "%Y:%m:%d %H:%M:%S").ok())
    }

    /// 设置图片的修改时间
    pub fn set_time(&mut self, time: NaiveDateTime) -> AnyResult<()> {
        let time_str = time.format("%Y:%m:%d %H:%M:%S").to_string();
        self.meta.set_tag_string(EXIF_TAG_DATE_TIME, &time_str)?;
        Ok(())
    }

    /// 获取图片的 GPS 经纬度坐标
    pub fn get_gps_coordinates(&self) -> Option<GpsInfo> {
        self.meta.get_gps_info()
    }

    /// 设置图片的 GPS 经纬度坐标
    pub fn set_gps_coordinates(&mut self, coordinates: GpsInfo) -> AnyResult<()> {
        Ok(self.meta.set_gps_info(&coordinates)?)
    }

    /// 获取图片的注释信息
    pub fn get_comment(&self) -> Option<String> {
        self.meta.get_tag_string(EXIF_TAG_USER_COMMENT).ok()
    }

    /// 设置图片的注释信息
    pub fn set_comment(&mut self, comment: &str) -> AnyResult<()> {
        self.meta.set_tag_string(EXIF_TAG_USER_COMMENT, comment)?;
        Ok(())
    }

    /// 保存元数据到文件
    pub fn save(&self, file_path: impl AsRef<Path>) -> AnyResult<()> {
        self.meta.save_to_file(file_path.as_ref())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_image() -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        let image_data = include_bytes!("../../../assets/images/lena.jpg");
        file.write_all(image_data).unwrap();
        file
    }

    #[test]
    fn test_get_modification_time() {
        let test_image = create_test_image();
        let exif_manager = ExifManager::new(test_image.path().to_str().unwrap()).unwrap();
        let modification_time = exif_manager.get_time();
        assert!(modification_time.is_none());
    }

    #[test]
    fn test_set_modification_time() {
        let test_image = create_test_image();
        let mut exif_manager = ExifManager::new(test_image.path()).unwrap();
        let new_time =
            NaiveDateTime::parse_from_str("2023:10:01 12:00:00", "%Y:%m:%d %H:%M:%S").unwrap();
        exif_manager.set_time(new_time).unwrap();
        exif_manager.save(test_image.path()).unwrap();
        let updated_time = exif_manager.get_time().unwrap();
        assert_eq!(updated_time, new_time);
    }

    #[test]
    fn test_get_gps_coordinates() {
        let test_image = create_test_image();
        let exif_manager = ExifManager::new(test_image).unwrap();
        let gps_coordinates = exif_manager.get_gps_coordinates();
        assert!(gps_coordinates.is_none());
    }

    #[test]
    fn test_set_gps_coordinates() {
        let test_image = create_test_image();
        let mut exif_manager = ExifManager::new(&test_image).unwrap();
        let new_coordinates = GpsInfo {
            latitude: 37.0,
            longitude: -122.0,
            altitude: 0.0,
        };
        exif_manager.set_gps_coordinates(new_coordinates).unwrap();
        exif_manager.save(test_image).unwrap();
        let updated_coordinates = exif_manager.get_gps_coordinates().unwrap();
        assert_eq!(updated_coordinates.latitude, new_coordinates.latitude);
        assert_eq!(updated_coordinates.longitude, new_coordinates.longitude);
    }

    #[test]
    fn test_get_comment() {
        let test_image = create_test_image();
        let exif_manager = ExifManager::new(test_image).unwrap();
        let comment = exif_manager.get_comment();
        assert!(comment.is_none());
    }

    #[test]
    fn test_set_comment() {
        let test_image = create_test_image();
        let mut exif_manager = ExifManager::new(&test_image).unwrap();
        let new_comment = "This is a test comment";
        exif_manager.set_comment(new_comment).unwrap();
        exif_manager.save(test_image).unwrap();
        let updated_comment = exif_manager.get_comment().unwrap();
        assert_eq!(updated_comment, new_comment);
    }
}
