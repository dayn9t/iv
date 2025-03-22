use crate::geo::SizeT;
pub use geo_types::CoordNum;

pub trait ISize2D<T: CoordNum> {
    /// 返回宽度尺寸
    fn width(&self) -> T;

    /// 返回高度尺寸
    fn height(&self) -> T;

    /// 获取尺寸
    fn size(&self) -> SizeT<T> {
        SizeT {
            width: self.width(),
            height: self.height(),
        }
    }
    /// 获取尺寸的面积
    fn area(&self) -> T {
        self.width() * self.height()
    }
}

impl<T: CoordNum> ISize2D<T> for SizeT<T> {
    fn width(&self) -> T {
        self.width
    }

    fn height(&self) -> T {
        self.height
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    // 辅助函数，用于测试任何实现了ISize2D的类型
    fn test_isize2d<T, S>(size: &S, expected_width: T, expected_height: T)
    where
        T: CoordNum,
        S: ISize2D<T>,
    {
        assert_eq!(size.width(), expected_width);
        assert_eq!(size.height(), expected_height);

        let actual_size = size.size();
        assert_eq!(actual_size.width, expected_width);
        assert_eq!(actual_size.height, expected_height);
    }

    #[test]
    fn test_size_t_implements_isize2d_i32() {
        let size = SizeT {
            width: 10,
            height: 20,
        };
        test_isize2d(&size, 10, 20);
    }

    #[test]
    fn test_size_t_implements_isize2d_f64() {
        let size = SizeT {
            width: 10.5,
            height: 20.5,
        };
        test_isize2d(&size, 10.5, 20.5);
    }

    #[test]
    fn test_empty_size() {
        let size = SizeT {
            width: 0,
            height: 0,
        };
        test_isize2d(&size, 0, 0);
    }

    #[test]
    fn test_negative_dimensions() {
        let size = SizeT {
            width: -5,
            height: -10,
        };
        test_isize2d(&size, -5, -10);
    }

    #[test]
    fn test_custom_type_implements_isize2d() {
        // 自定义类型实现ISize2D
        struct CustomSize {
            w: u32,
            h: u32,
        }

        impl ISize2D<u32> for CustomSize {
            fn width(&self) -> u32 {
                self.w
            }

            fn height(&self) -> u32 {
                self.h
            }
        }

        let custom = CustomSize { w: 100, h: 200 };
        test_isize2d(&custom, 100, 200);
    }
}
