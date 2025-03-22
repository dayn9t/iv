pub use crate::image::Rgb;
use iv_core::geo::Rect;
use iv_core::geo::iface::ISize2D;
use rx_core::prelude::AnyResult;
use std::path::Path;

/// 2D 图像接口
pub trait IImage2D: ISize2D<i32> + Sized {
    /// 获取图像上指定矩形区域的子图像
    ///
    /// 自动裁剪超出图像边界的部分
    fn get_roi(&self, rect: Rect) -> Self;

    /// 设置图像上指定矩形区域的子图像
    fn set_roi(&mut self, rect: Rect, other: &Self);

    /// 填充全图颜色
    fn fill_color(&mut self, color: Rgb);

    /// 缩放到目标图形中
    fn resize_to(&self, dst: &mut Self);

    /// 加在图像
    fn load(path: &Path) -> AnyResult<Self>;
    /// 保存图像
    fn save(&self, path: &Path) -> AnyResult<()>;
}
