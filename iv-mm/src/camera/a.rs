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

const PIXEL_FORMAT_MJPEG: PixelFormat = PixelFormat::new(u32::from_le_bytes([b'M', b'J', b'P', b'G']), 0);
const PIXEL_FORMAT_YUYV: PixelFormat = PixelFormat::new(u32::from_le_bytes([b'Y', b'U', b'Y', b'V']), 0);
const SIZE_NHD: Size = Size { width: 640, height: 360 };

const RECV_TIMEOUT: Duration = Duration::from_secs(2);

pub struct CameraA<'a> {
    camera: libcamera::camera::ActiveCamera<'a>,
    stream: libcamera::stream::Stream,
    buffers: Vec<MemoryMappedFrameBuffer<FrameBuffer>>,
    tx: std::sync::mpsc::Sender<libcamera::request::Request>,
    rx: std::sync::mpsc::Receiver<libcamera::request::Request>,
}

impl<'a> CameraA<'_> {
    pub fn new() -> Self {
        let mgr = CameraManager::new().unwrap();
        let cameras = mgr.cameras();
        let camera = cameras.get(0).expect("No cameras found");
        let cam_model = camera.properties().get::<properties::Model>().unwrap();
        println!("Using camera: {}", *cam_model);
        let mut camera = camera.acquire().expect("Unable to acquire camera");

        let mut cfgs = camera.generate_configuration(&[StreamRole::VideoRecording]).unwrap();
        let mut c = cfgs.get_mut(0).unwrap();
        c.set_pixel_format(PIXEL_FORMAT_YUYV);
        c.set_size(SIZE_NHD);
        println!("Generated config: {:#?}", cfgs);
        match cfgs.validate() {
            CameraConfigurationStatus::Valid => println!("Camera configuration valid!"),
            CameraConfigurationStatus::Adjusted => println!("Camera configuration was adjusted: {:#?}", cfgs),
            CameraConfigurationStatus::Invalid => panic!("Error validating camera configuration"),
        }
        camera.configure(&mut cfgs).expect("Unable to configure camera");

        let mut alloc = FrameBufferAllocator::new(&camera);
        let cfg = cfgs.get(0).unwrap();
        let stream = cfg.stream().unwrap();
        let buffers = alloc.alloc(&stream).unwrap();
        println!("Allocated {} buffers", buffers.len());

        let buffers = buffers
            .into_iter()
            .map(|buf| MemoryMappedFrameBuffer::new(buf).unwrap())
            .collect::<Vec<_>>();

        let (tx, rx) = std::sync::mpsc::channel();
        camera.on_request_completed(move |req| {
            tx.send(req).unwrap();
        });

        CameraA {
            camera,
            stream,
            buffers,
            tx,
            rx,
        }
    }

    pub fn start(&mut self) {
        self.camera.start(None).unwrap();
        for (i, buf) in self.buffers.iter().enumerate() {
            let mut req = self.camera.create_request(Some(i as u64)).unwrap();
            req.add_buffer(&self.stream, buf).unwrap();
            self.camera.queue_request(req).unwrap();
        }
    }

    pub fn read(&mut self) -> Option<Vec<u8>> {
        let mut req = self.rx.recv_timeout(RECV_TIMEOUT).ok()?;
        let framebuffer: &MemoryMappedFrameBuffer<FrameBuffer> = req.buffer(&self.stream).unwrap();
        let planes = framebuffer.data();
        let frame_data = planes.get(0).unwrap();
        let bytes_used = framebuffer.metadata().unwrap().planes().get(0).unwrap().bytes_used as usize;
        let data = frame_data[..bytes_used].to_vec();
        req.reuse(ReuseFlag::REUSE_BUFFERS);
        self.camera.queue_request(req).unwrap();
        Some(data)
    }
}