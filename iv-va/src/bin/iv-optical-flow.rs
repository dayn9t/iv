use std::time;

use opencv::{core::{self, GpuMat}, cudaimgproc, cudawarping, cudaoptflow, highgui, imgcodecs, imgproc, prelude::*, Result, videoio};
use opencv::core::{Size, Ptr, Point, Scalar, cart_to_polar};
use opencv::imgproc::{INTER_LINEAR, line, LINE_8};

/// 光流分析器
struct OpticalFlowAnalyzer {
    optical_flow: Ptr<dyn opencv::prelude::CUDA_FarnebackOpticalFlow>,
    small: GpuMat,
    gray0: GpuMat,
    gray1: GpuMat,
}

const SCALE: i32 = 8;
const R2: f32 = 2.0;

impl OpticalFlowAnalyzer {
    /// 构造光流分析器
    pub fn create() -> Result<Self> {
        let num_levels: i32 = 5;
        let pyr_scale: f64 = 0.5;
        let fast_pyramids: bool = false;
        let win_size: i32 = 13;
        let num_iters: i32 = 10;
        let poly_n: i32 = 5;
        let poly_sigma: f64 = 1.1;
        let flags: i32 = 0;
        #[warn(bare_trait_objects)]
        let optical_flow = <dyn cudaoptflow::CUDA_FarnebackOpticalFlow>::create(num_levels, pyr_scale, fast_pyramids, win_size, num_iters, poly_n, poly_sigma, flags)?;
        let a = OpticalFlowAnalyzer {
            optical_flow,
            small: GpuMat::default()?,
            gray0: GpuMat::default()?,
            gray1: GpuMat::default()?,
        };
        Ok(a)
    }

    /// 计算光流
    fn calc(&mut self, src: &GpuMat, dst: &mut GpuMat, stream: &mut core::Stream) -> Result<bool>
    {
        let scale = 1.0 / (SCALE as f64);
        cudawarping::resize(&src, &mut self.small, Size::default(), scale, scale, INTER_LINEAR, stream)?;
        cudaimgproc::cvt_color(&self.small, &mut self.gray1, imgproc::COLOR_BGR2GRAY, 0, stream)?;

        let ok = if self.gray0.empty()? {
            false
        } else {
            self.optical_flow.calc(&self.gray0, &self.gray1, dst, stream)?;
            true
        };
        std::mem::swap(&mut self.gray0, &mut self.gray1);
        Ok(ok)
    }
}

/// 加载图片到GPU
fn _load_to_gpu(file: &str) -> Result<GpuMat> {
    let img = imgcodecs::imread(file, imgcodecs::IMREAD_COLOR)?;
    let mut img_gpu = GpuMat::default()?;
    img_gpu.upload(&img)?;
    Ok(img_gpu)
}


fn draw_flow(flow: &GpuMat, frame: &mut Mat) -> Result<()> {
    // println!("flow_gpu type={:?}  size={:?} channels={:?}", flow.typ()?, flow.size()?, flow.channels()?);

    let mut flow2 = Mat::default();
    flow.download(&mut flow2)?;

    // 转换成极坐标
    // let mut mag = Mat::default();
    // let mut angle = Mat::default();
    // cart_to_polar(&flow2, &flow2, &mut mag, &mut angle, true);

    // TODO: 用小图测试清楚运功矢量的正反

    /*
    for y in 0..flow2.rows() {
        let row = flow2.at_row::<Vec2b<f32>>(y)?;
        //println!("#{:?} row size: {:?}", i, row.len());
        for x in 0..row.len() {
            let r2 = row[x][0] * row[x][0] + row[x][1] * row[x][1];
            if r2 > R2 {
                let scale = SCALE as f32 * 1.0;
                let p0 = Point::new(x as i32 * SCALE, y as i32 * SCALE);
                let x1 = p0.x + (row[x][0] * scale) as i32;
                let y1 = p0.y + (row[x][1] * scale) as i32;
                let p1 = Point::new(x1, y1);
                let color = Scalar::new(0.0, 255.0, 0.0, 0.0);
                line(frame, p0, p1, color, 1, LINE_8, 0)?;

                //println!("#{:?}\t{:?}\t{:?}", x, row[x], r2);
            }
        }
    }

    for y in 0..flow2.rows() {
        let row = flow2.at_row::<Vec2<f32>>(y)?;
        //println!("#{:?} row size: {:?}", i, row.len());
        for x in 0..row.len() {
            let r2 = row[x][0] * row[x][0] + row[x][1] * row[x][1];
            if r2 > R2 {
                let scale = SCALE as f32 * 1.0;
                let p0 = Point::new(x as i32 * SCALE, y as i32 * SCALE);
                let x1 = p0.x + (row[x][0] * scale) as i32;
                let y1 = p0.y + (row[x][1] * scale) as i32;
                let _p1 = Point::new(x1, y1);

                let color = Scalar::new(0.0, 0.0, 255.0, 0.0);
                line(frame, p0, p0, color, 1, LINE_8, 0)?;

                //println!("#{:?}\t{:?}\t{:?}", x, row[x], r2);
            }
        }
    }
    */
    Ok(())
}

fn main() -> Result<()> {
    let dev_count = core::get_cuda_enabled_device_count()?;
    for dev_num in 0..dev_count {
        core::print_short_cuda_device_info(dev_num)?;
    }
    let mut stream = core::Stream::default()?;
    let mut analyer = OpticalFlowAnalyzer::create()?;

    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    let file = "/home/jiang/ws/lift/098.mp4";
    let mut cam = videoio::VideoCapture::from_file(file, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    let mut frame = Mat::default();
    let mut src = GpuMat::default()?;
    let mut flow = GpuMat::default()?;
    let mut i = 0;
    loop {
        let ok = cam.read(&mut frame)?;
        i += 1;
        if i % 5 != 0 {
            continue;
        }
        println!("#{:?}", i);

        if ok {
            src.upload(&frame)?;
            let ok = analyer.calc(&src, &mut flow, &mut stream)?;

            stream.wait_for_completion()?;

            draw_flow(&flow, &mut frame)?;
            highgui::imshow(window, &mut frame)?;
        }
        let key = highgui::wait_key(0)?;
        if key == 27 {
            break;
        }
    }
    Ok(())
}
