use std::io::BufReader;

use std::path::Path;
use std::thread::sleep;

use rodio::Source;

pub fn play(path: &Path) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = std::fs::File::open(path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
}

pub fn play_wait(path: &Path) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = std::fs::File::open(path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let duration = source.total_duration().unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    sleep(duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let p = Path::new("/home/jiang/ws/bot/sound/211-218_1.mp3");
        play_wait(p);
    }
}
