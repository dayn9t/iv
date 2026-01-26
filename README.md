# jsr-video-subs

视频字幕合成工具 - 将多个 SRT 字幕叠加到视频中，用于对比不同字幕源的准确度。

## 功能

- 多个 SRT 字幕文件按顺序（由下往上）等距排列
- 字幕燃烧（burn-in）到视频画面
- 固定 H.264 输出编码
- 可配置字体大小、颜色、间距

## 安装

### 系统依赖

需要安装 FFmpeg（>= 4.0）：

```bash
# Ubuntu/Debian
sudo apt install ffmpeg

# macOS
brew install ffmpeg

# Arch Linux
sudo pacman -S ffmpeg
```

### 编译安装

```bash
cargo install --path .
```

## 使用

### 基本用法

```bash
jsr-video-subs -i input.mp4 -o output.mp4 -s zh.srt,en.srt
```

### 完整参数

```bash
jsr-video-subs \
  --input input.mp4 \
  --output output.mp4 \
  --subs sub1.srt,sub2.srt,sub3.srt \
  --font-size 28 \
  --font-color yellow \
  --spacing 20 \
  --crf 20 \
  --verbose
```

### 参数说明

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `-i, --input` | 输入视频文件 | 必填 |
| `-o, --output` | 输出视频文件 | 必填 |
| `-s, --subs` | 字幕文件列表（逗号分隔） | 必填 |
| `--font-size` | 字体大小（像素） | 24 |
| `--font-color` | 字体颜色 | white |
| `--spacing` | 字幕间距 | =字体大小 |
| `--crf` | H.264 质量（0-51） | 23 |
| `--preset` | 编码速度预设 | medium |
| `-v, --verbose` | 详细输出 | false |

## 字幕排列

字幕从下往上等距排列：

```
┌─────────────────────┐
│                     │
│  字幕3 (top)        │
│                     │
│  字幕2              │
│                     │
│  字幕1 (bottom)     │
└─────────────────────┘
```

## 开发

```bash
# 构建
cargo build

# 运行
cargo run -- -i input.mp4 -o output.mp4 -s sub1.srt,sub2.srt

# 测试
cargo test
```
