use std::io::BufReader;

use cpal::traits::{DeviceTrait, HostTrait};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread::sleep;

use rodio::Source;

pub fn play_sound(path: &Path) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = std::fs::File::open(path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
}

pub fn play_sound_wait(path: &Path) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    println!("Speaker[default] play {:?}", path);
    let file = std::fs::File::open(path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let duration = source.total_duration().unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    sleep(duration);
}

pub fn play_sound_n_wait(path: &Path, device_index: usize) {
    println!("Speaker[{}] play {:?}", device_index, path);
    // 获取默认的音频主机
    let host = cpal::default_host();
    let mut devices = host.output_devices().unwrap();
    let device = devices
        .nth(device_index)
        .expect("No such output device available");

    let (_stream, stream_handle) = rodio::OutputStream::try_from_device(&device).unwrap();

    let file = std::fs::File::open(path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let duration = source.total_duration().unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    println!("sleep duration: {:?}", duration);
    sleep(duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let p = Path::new("/home/jiang/ws/bot/sounds/app-start.wav");
        play_sound_wait(p);
    }
}
