//pub mod camera;

//mod a;

use std::{fs::OpenOptions, io::Write, process::exit, time::Duration};
use std::ffi::c_void;

use libcamera::{
    camera::CameraConfigurationStatus,
    camera_manager::CameraManager,
    framebuffer::AsFrameBuffer,
    framebuffer_allocator::{FrameBuffer, FrameBufferAllocator},
    framebuffer_map::MemoryMappedFrameBuffer,
    pixel_format::PixelFormat,
    properties,
    request::ReuseFlag,
    stream::StreamRole,
};
use libcamera::geometry::Size;
use opencv::{core, highgui, imgproc, prelude::*};

// drm-fourcc does not have MJPEG type yet, construct it from raw fourcc identifier
const PIXEL_FORMAT_MJPEG: PixelFormat = PixelFormat::new(u32::from_le_bytes([b'M', b'J', b'P', b'G']), 0);
const PIXEL_FORMAT_YUYV: PixelFormat = PixelFormat::new(u32::from_le_bytes([b'Y', b'U', b'Y', b'V']), 0);
const SIZE_NHD: Size = Size { width: 640, height: 360 };

const RECV_TIMEOUT: Duration = Duration::from_secs(2);


pub fn show(frame_data: &[u8])
{
    unsafe {
        let width = 640; // 图像的宽度
        let height = 360; // 图像的高度

        let data_ptr = frame_data.as_ptr() as *mut c_void;

        // 创建一个Mat对象，用于存储YUYV格式的图像
        let yuyv_mat = Mat::new_rows_cols_with_data_unsafe_def(height, width, core::CV_8UC2, data_ptr).unwrap();

        // 创建一个Mat对象，用于存储转换后的BGR格式的图像
        let mut bgr_mat = Mat::new_size(yuyv_mat.size().unwrap(), core::CV_8UC3).unwrap();

        // 将YUYV格式转换为BGR格式
        imgproc::cvt_color(&yuyv_mat, &mut bgr_mat, imgproc::COLOR_YUV2BGR_YUYV, 0).unwrap();

        // 使用highgui显示图像
        highgui::imshow("BGR Image", &bgr_mat).unwrap();
        highgui::wait_key(30).unwrap();
    }
}


pub fn main1() {
    let filename = match std::env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("Error: missing file output parameter");
            println!("Usage: ./video_capture </path/to/output.mjpeg>");
            exit(1);
        }
    };
    // 是想头获取
    let mgr = CameraManager::new().unwrap();
    let cameras = mgr.cameras();
    // 程序选择第一个摄像头
    let camera = cameras.get(0).expect("No cameras found");
    let cam_model = camera.properties().get::<properties::Model>().unwrap();
    println!("Using camera: {}", *cam_model);
    let mut active_camera = camera.acquire().expect("Unable to acquire camera");

    // 摄像头配置
    // This will generate default configuration for each specified role
    let mut cfgs = active_camera.generate_configuration(&[StreamRole::VideoRecording]).unwrap();
    let mut c = cfgs.get_mut(0).unwrap();
    c.set_pixel_format(PIXEL_FORMAT_YUYV);
    c.set_size(SIZE_NHD);
    println!("Generated config: {:#?}", cfgs);
    match cfgs.validate() {
        CameraConfigurationStatus::Valid => println!("Camera configuration valid!"),
        CameraConfigurationStatus::Adjusted => println!("Camera configuration was adjusted: {:#?}", cfgs),
        CameraConfigurationStatus::Invalid => panic!("Error validating camera configuration"),
    }
    active_camera.configure(&mut cfgs).expect("Unable to configure camera");

    // 分配缓冲区
    let mut alloc = FrameBufferAllocator::new(&active_camera);
    // Allocate frame buffers for the stream
    let cfg = cfgs.get(0).unwrap();
    let stream = cfg.stream().unwrap();
    let buffers = alloc.alloc(&stream).unwrap();
    println!("Allocated {} buffers", buffers.len());

    // Convert FrameBuffer to MemoryMappedFrameBuffer, which allows reading &[u8]
    let buffers = buffers
        .into_iter()
        .map(|buf| MemoryMappedFrameBuffer::new(buf).unwrap())
        .collect::<Vec<_>>();

    // 程序为每个帧缓冲区创建一个捕获请求，并将其添加到请求
    let reqs = buffers
        .into_iter()
        .enumerate()
        .map(|(i, buf)| {
            let mut req = active_camera.create_request(Some(i as u64)).unwrap();
            req.add_buffer(&stream, buf).unwrap();
            req
        })
        .collect::<Vec<_>>();

    // Completed capture requests are returned as a callback
    let (tx, rx) = std::sync::mpsc::channel();
    active_camera.on_request_completed(move |req| {
        tx.send(req).unwrap();
    });

    // TODO: 调节帧率, Set `Control::FrameDuration()` here. Blocked on https://github.com/lit-robotics/libcamera-rs/issues/2
    active_camera.start(None).unwrap();

    // 所有请求排入队列以供执行
    for req in reqs {
        //println!("Request queued for execution: {req:#?}");
        active_camera.queue_request(req).unwrap();
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filename)
        .expect("Unable to create output file");

    let mut count = 0;
    while count < 160 {
        //println!("Waiting for camera request execution");
        let mut req = rx.recv_timeout(RECV_TIMEOUT).expect("Camera request failed");

        //println!("Camera request {:?} completed!", req);
        //println!("Metadata: {:#?}", req.metadata());

        // Get framebuffer for our stream
        let framebuffer: &MemoryMappedFrameBuffer<FrameBuffer> = req.buffer(&stream).unwrap();
        println!("FrameBuffer metadata: {:#?}", framebuffer.metadata());

        // MJPEG format has only one data plane containing encoded jpeg data with all the headers
        let planes = framebuffer.data();
        let frame_data = planes.get(0).unwrap();
        // Actual encoded data will be smalled than framebuffer size, its length can be obtained from metadata.
        let bytes_used = framebuffer.metadata().unwrap().planes().get(0).unwrap().bytes_used as usize;

        show(&frame_data[..bytes_used]);

        file.write(&frame_data[..bytes_used]).unwrap();
        println!("Written {} bytes to {}", bytes_used, &filename);

        // Recycle the request back to the camera for execution
        req.reuse(ReuseFlag::REUSE_BUFFERS);
        active_camera.queue_request(req).unwrap();

        count += 1;
    }

    // Everything is cleaned up automatically by Drop implementations
}
