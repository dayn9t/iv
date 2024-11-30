# IV-MM

多媒体相关功能, 图片/视频/音频等.

- opencv-rust
  - [Changes](https://github.com/twistedfall/opencv-rust/blob/master/CHANGES.md) 
  - 依赖: ``````

## 工具

- 录音该工具: audacity

## 依赖

- 媒体文件元信息: ```libgexiv2-dev```
- 声音播放: ```libasound2-dev``` 
- opencv: ```clang libclang-dev ninja-build libopencv-imgproc-dev libopencv-highgui-dev``` # libopencv-dev
- libcamera: ```libcamera-dev```

## 工具

获取时间长度: 
```bash
ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1 a.mp3
# 3786.448980
ffmpeg -i 德云社_01-03-06.mp3 2>&1 | grep Duration
# Duration: 01:03:06.45, start: 0.025057, bitrate: 128 kb/s
```

