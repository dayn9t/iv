use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::channel;

pub fn print_input_db() {
    // 获取默认的音频主机
    let host = cpal::default_host();

    // 获取默认的输入设备
    let input_device = host
        .default_input_device()
        .expect("Failed to get default input device");
    let input_config = input_device
        .default_input_config()
        .expect("Failed to get default input config");

    // 创建输入流
    let (tx, rx) = channel();
    let input_stream = input_device
        .build_input_stream(
            &input_config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                // 计算分贝
                let db = 20.0
                    * data
                        .iter()
                        .fold(0.0, |acc, &sample| acc + sample.abs().log10())
                    / data.len() as f32;

                // 发送分贝到主线程
                tx.send(db).unwrap();
            },
            |err| eprintln!("an error occurred on input stream: {}", err),
            None, // 添加的 Option<Duration> 参数
        )
        .unwrap();

    // 播放输入流
    input_stream.play().unwrap();

    // 在主线程中打印分贝
    loop {
        let db = rx.recv().unwrap();
        println!("Input dB: {}", db);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        print_input_db();
    }
}
