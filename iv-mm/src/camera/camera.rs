use std::sync::mpsc::Receiver;
use std::time::Duration;

pub use libcamera::camera::Camera;
pub use libcamera::camera_manager::CameraManager;
use libcamera::{
    camera::{CameraConfiguration, CameraConfigurationStatus},
    framebuffer::AsFrameBuffer,
    framebuffer_allocator::{FrameBuffer, FrameBufferAllocator},
    framebuffer_map::MemoryMappedFrameBuffer,
    pixel_format::PixelFormat,
    properties,
    request::{Request, ReuseFlag},
    stream::StreamRole,
};

use iv_core::geo::Size;

mod c {
    pub use libcamera::camera::ActiveCamera;
    pub use libcamera::geometry::Size;
}

// drm-fourcc does not have MJPEG type yet, construct it from raw fourcc identifier
pub const PIXEL_FORMAT_MJPEG: PixelFormat =
    PixelFormat::new(u32::from_le_bytes([b'M', b'J', b'P', b'G']), 0);
pub const PIXEL_FORMAT_YUYV: PixelFormat =
    PixelFormat::new(u32::from_le_bytes([b'Y', b'U', b'Y', b'V']), 0);

const RECV_TIMEOUT: Duration = Duration::from_secs(2);

/// Size转换
pub fn c_size(s: Size) -> c::Size {
    c::Size {
        width: s.width as u32,
        height: s.height as u32,
    }
}

pub struct ActiveCamera<'d> {
    active_camera: c::ActiveCamera<'d>,
    configs: CameraConfiguration,
    receiver: Option<Receiver<Request>>,
    last_timestamp: u64,
    interval: u64,
}

impl<'d> ActiveCamera<'d> {
    /// 创建摄像机
    pub fn new(camera: &'d Camera<'_>, format: PixelFormat, size: Size, fps: f32) -> Self {
        let mut active_camera = camera.acquire().expect("Unable to acquire camera");

        // 摄像头配置
        // This will generate default configuration for each specified role
        let mut configs = active_camera
            .generate_configuration(&[StreamRole::VideoRecording])
            .unwrap();
        let mut c = configs.get_mut(0).unwrap();
        c.set_pixel_format(format);
        c.set_size(c_size(size));
        println!("Generated config: {:#?}", configs);
        match configs.validate() {
            CameraConfigurationStatus::Valid => println!("Camera configuration valid!"),
            CameraConfigurationStatus::Adjusted => {
                println!("Camera configuration was adjusted: {:#?}", configs)
            }
            CameraConfigurationStatus::Invalid => panic!("Error validating camera configuration"),
        }
        active_camera
            .configure(&mut configs)
            .expect("Unable to configure camera");

        Self {
            active_camera,
            configs,
            receiver: None,
            last_timestamp: 0,
            interval: (1000_000_000f32 / fps) as u64,
        }
    }

    /// 启动摄像机
    pub fn start(&mut self) {
        // 分配缓冲区
        let mut alloc = FrameBufferAllocator::new(&self.active_camera);
        // Allocate frame buffers for the stream
        let cfg = self.configs.get(0).unwrap();
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
                let mut req = self.active_camera.create_request(Some(i as u64)).unwrap();
                req.add_buffer(&stream, buf).unwrap();
                req
            })
            .collect::<Vec<_>>();

        // Completed capture requests are returned as a callback
        let (tx, rx) = std::sync::mpsc::channel();
        self.active_camera.on_request_completed(move |req| {
            tx.send(req).unwrap();
        });

        // TODO: 调节帧率, Set `Control::FrameDuration()` here. Blocked on https://github.com/lit-robotics/libcamera-rs/issues/2
        self.active_camera.start(None).unwrap();

        // 所有请求排入队列以供执行
        for req in reqs {
            //println!("Request queued for execution: {req:#?}");
            self.active_camera.queue_request(req).unwrap();
        }
        self.receiver = Some(rx);
    }

    /// 读取帧数据到缓冲区
    pub fn read(&mut self, buffer: &mut Vec<u8>) {
        //println!("Waiting for camera request execution");
        let receiver = self.receiver.as_ref().unwrap();
        let mut got = false;
        while !got {
            let mut req = receiver
                .recv_timeout(RECV_TIMEOUT)
                .expect("Camera request failed");

            //println!("Camera request {:?} completed!", req);
            //println!("Metadata: {:#?}", req.metadata());

            // Get framebuffer for our stream
            let cfg = self.configs.get(0).unwrap();
            let stream = cfg.stream().unwrap();
            let framebuffer: &MemoryMappedFrameBuffer<FrameBuffer> = req.buffer(&stream).unwrap();
            let meta = framebuffer.metadata().unwrap();
            let timestamp = meta.timestamp();
            if timestamp >= self.last_timestamp + self.interval {
                println!("timestamp: {:#?}", timestamp);
                //println!("FrameBuffer metadata: {:#?}", framebuffer.metadata());

                let planes = framebuffer.data();
                let frame_data = planes.get(0).unwrap();
                let bytes_used = meta.planes().get(0).unwrap().bytes_used as usize;

                buffer.clear();
                buffer.extend_from_slice(&frame_data[..bytes_used]);

                self.last_timestamp = timestamp;
                got = true;
            } else {
                println!("timestamp: {:#?} skip", timestamp);
            }

            // 复用缓冲区
            req.reuse(ReuseFlag::REUSE_BUFFERS);
            self.active_camera.queue_request(req).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
