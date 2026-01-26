# 音频频谱视频生成功能设计

**日期**: 2026-01-27
**状态**: 设计

## 目标

将 MP3 音频文件转换为 1280x720 的 MP4 视频，画面为纯黑背景上的 64 根蓝色柱状频谱动画。

## 功能规格

| 项目 | 配置 |
|------|------|
| 输入 | MP3 音频文件 |
| 输出 | 1280x720 MP4 视频 |
| 帧率 | 25 fps |
| 画面 | 纯黑背景 + 蓝色柱状频谱 |
| 频谱柱数量 | 64 根 |
| 视频编码 | H.264 (CRF 23) |
| 音频编码 | AAC |

## 实现方式

### FFmpeg 滤镜链

```
color=c=black:s=1280x720:d={duration} [bg];
[0:a] showspectrum=s=1280x720:mode=separate:slide=scroll:win_func=hann:overlap=0.8:colors=blue#0000FF [v];
[bg][v] overlay=format=auto
```

**滤镜说明**：
- `color` - 创建纯黑背景，持续时间为音频时长
- `showspectrum` - 生成频谱动画
  - `mode=separate` - 声道分离
  - `slide=scroll` - 滚动模式
  - `win_func=hann` - 汉宁窗，减少频谱泄漏
  - `overlap=0.8` - 帧重叠，更平滑
  - `colors=blue#0000FF` - 蓝色单色
- `overlay` - 将频谱叠加到黑背景上

## 代码架构

### 新增模块

**`src/audio_video.rs`**

```rust
use anyhow::Result;
use std::path::Path;

pub fn audio_to_spectrum_video(
    audio: &Path,
    output: &Path,
    width: u32,
    height: u32,
    fps: u32,
    crf: u32,
    preset: &str,
    verbose: bool,
) -> Result<()>
```

**实现流程**：
1. 使用 FFprobe 获取音频时长
2. 根据时长构建滤镜链
3. 调用 FFmpeg 执行转换
4. 验证输出状态

### CLI 集成

**新增子命令**：`audio-video`

```bash
jsr-video-subs audio-video -i audio.mp3 -o video.mp4
```

**新增参数**：
| 参数 | 说明 | 默认值 |
|------|------|--------|
| `--width` | 视频宽度 | 1280 |
| `--height` | 视频高度 | 720 |
| `--fps` | 帧率 | 25 |
| `--crf` | 质量 | 23 |

**命令结构**：使用 Clap 的 `Subcommand` 枚举，与现有 `subs` 功能并列。

### 文件结构

```
src/
├── main.rs          # 添加 Subcommand 枚举
├── processor.rs     # 现有字幕处理
├── subtitle.rs      # 现有 SRT 转换
└── audio_video.rs   # 新增音频转视频
```

## 测试策略

**单元测试**：
- FFprobe 时长解析测试
- 滤镜链生成测试

**集成测试**：
- 使用短音频片段（1-2秒）验证输出
- 验证输出视频分辨率和帧率

**测试资源**：
- `tests/fixtures/short.mp3` - 测试用音频

## 依赖

无需新增 Rust 依赖。

系统依赖：
- FFmpeg >= 4.0（已有 `showspectrum` 滤镜）
- FFprobe（通常随 FFmpeg 一起安装）
