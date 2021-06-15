use std::time;

use opencv::{core::{self, GpuMat}, cudaimgproc, cudawarping, cudaoptflow, highgui, imgcodecs, imgproc, prelude::*, Result};
use opencv::core::{Size, Ptr, Vec2, Point, Scalar};
use opencv::imgproc::{INTER_LINEAR, line, LINE_8};

/// 光流分析器
struct OpticalFlowAnalyzer {
    optical_flow: Ptr<dyn opencv::prelude::CUDA_FarnebackOpticalFlow>,
    small: GpuMat,
    gray0: GpuMat,
    gray1: GpuMat,
}

const SCALE: i32 = 4;

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
        let optical_flow = cudaoptflow::CUDA_FarnebackOpticalFlow::create(num_levels, pyr_scale, fast_pyramids, win_size, num_iters, poly_n, poly_sigma, flags)?;
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
fn load_to_gpu(file: &str) -> Result<GpuMat> {
    let img = imgcodecs::imread(file, imgcodecs::IMREAD_COLOR)?;
    let mut img_gpu = GpuMat::default()?;
    img_gpu.upload(&img)?;
    Ok(img_gpu)
}


fn draw_flow(flow: &GpuMat, frame: &mut Mat) -> Result<()> {
    println!("flow_gpu type={:?}  size={:?} channels={:?}", flow.typ()?, flow.size()?, flow.channels()?);

    let mut flow2 = Mat::default();
    flow.download(&mut flow2)?;

    for y in 0..flow2.rows() {
        let row = flow2.at_row::<Vec2<f32>>(y)?;
        //println!("#{:?} row size: {:?}", i, row.len());
        for x in 0..row.len() {
            let r2 = row[x][0] * row[x][0] + row[x][1] * row[x][1];
            if r2 > 4.0 {
                let scale = SCALE as f32 * 1.0;
                let p0 = Point::new(x as i32 * SCALE, y as i32 * SCALE);
                let x1 = p0.x + (row[x][0] * scale) as i32;
                let y1 = p0.y + (row[x][1] * scale) as i32;
                let p1 = Point::new(x1, y1);
                let color = Scalar::new(0.0, 255.0, 0.0, 0.0);
                line(frame, p0, p1, color, 1, LINE_8, 0)?;

                println!("#{:?}\t{:?}\t{:?}", x, row[x], r2);
            }
        }
    }

    for y in 0..flow2.rows() {
        let row = flow2.at_row::<Vec2<f32>>(y)?;
        //println!("#{:?} row size: {:?}", i, row.len());
        for x in 0..row.len() {
            let r2 = row[x][0] * row[x][0] + row[x][1] * row[x][1];
            if r2 > 4.0 {
                let scale = SCALE as f32 * 1.0;
                let p0 = Point::new(x as i32 * SCALE, y as i32 * SCALE);
                let x1 = p0.x + (row[x][0] * scale) as i32;
                let y1 = p0.y + (row[x][1] * scale) as i32;
                let p1 = Point::new(x1, y1);

                let color = Scalar::new(0.0, 0.0, 255.0, 0.0);
                line(frame, p1, p1, color, 1, LINE_8, 0)?;

                println!("#{:?}\t{:?}\t{:?}", x, row[x], r2);
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let file1 = "/home/jiang/ws/nvidia/nano_test/data/basketball1.png";
    let file2 = "/home/jiang/ws/nvidia/nano_test/data/basketball2.png";

    let dev_count = core::get_cuda_enabled_device_count()?;
    for dev_num in 0..dev_count {
        core::print_short_cuda_device_info(dev_num)?;
    }
    let mut stream = core::Stream::default()?;

    let img1 = load_to_gpu(file1)?;
    let img2 = load_to_gpu(file2)?;
    let mut flow = GpuMat::default()?;

    let mut analyer = OpticalFlowAnalyzer::create()?;

    let ok = analyer.calc(&img1, &mut flow, &mut stream)?;
    println!("ok: {:?}", ok);

    let ok = analyer.calc(&img2, &mut flow, &mut stream)?;
    println!("ok: {:?}", ok);

    //let start = time::Instant::now();

    stream.wait_for_completion()?;


    //println!("{:#?}", start.elapsed());

    let mut frame = imgcodecs::imread(file1, imgcodecs::IMREAD_COLOR)?;
    println!("frame size: {:?}", frame.size());
    draw_flow(&flow, &mut frame)?;

    highgui::imshow("Image win", &frame)?;
    let _key = highgui::wait_key(0)?;

    Ok(())
}
