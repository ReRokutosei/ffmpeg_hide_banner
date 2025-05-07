# FFmpeg Launcher

[![Rust](https://img.shields.io/badge/language-Rust-dea584.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

一个简单的 FFmpeg 启动器，用于自动隐藏 FFmpeg 的 banner 信息。

## ⭐ 特性

- 自动为 FFmpeg 命令添加 `-hide_banner` 参数
- 零配置，即装即用
- 支持 ffmpeg 和 ffprobe
- 体积小巧（< 1MB）
- 支持参数完全透传

## 🔍 背景

在使用 FFmpeg 时，每次执行都会显示一大段版本和编译信息：

```sh
ffmpeg version 2024-11-21-git-f298507323-essentials_build-www.gyan.dev Copyright (c) 2000-2024 the FFmpeg developers
  built with gcc 14.2.0 (Rev1, Built by MSYS2 project)
  configuration: --enable-gpl --enable-version3 --enable-static --disable-w32threads --disable-autodetect --enable-fontconfig --enable-iconv --enable-gnutls --enable-libxml2 --enable-gmp --enable-bzlib --enable-lzma --enable-zlib --enable-libsrt --enable-libssh --enable-libzmq --enable-avisynth --enable-sdl2 --enable-libwebp --enable-libx264 --enable-libx265 --enable-libxvid --enable-libaom --enable-libopenjpeg --enable-libvpx --enable-mediafoundation --enable-libass --enable-libfreetype --enable-libfribidi --enable-libharfbuzz --enable-libvidstab --enable-libvmaf --enable-libzimg --enable-amf --enable-cuda-llvm --enable-cuvid --enable-dxva2 --enable-d3d11va --enable-d3d12va --enable-ffnvcodec --enable-libvpl --enable-nvdec --enable-nvenc --enable-vaapi --enable-libgme --enable-libopenmpt --enable-libopencore-amrwb --enable-libmp3lame --enable-libtheora --enable-libvo-amrwbenc --enable-libgsm --enable-libopencore-amrnb --enable-libopus --enable-libspeex --enable-libvorbis --enable-librubberband
  libavutil      59. 47.100 / 59. 47.100
  libavcodec     61. 25.102 / 61. 25.102
  libavformat    61.  9.100 / 61.  9.100
  libavdevice    61.  4.100 / 61.  4.100
  libavfilter    10.  6.101 / 10.  6.101
  libswscale      8.  9.101 /  8.  9.101
  libswresample   5.  4.100 /  5.  4.100
  libpostproc    58.  4.100 / 58.  4.100
```

虽然可以通过 `-hide_banner` 参数隐藏这些信息，但每次都要手动添加比较繁琐。本项目通过一个轻量级的 Rust 程序优雅地解决这个问题。

## 📥 安装

1. 将原始的 `ffmpeg.exe` 重命名为 `ffmpeg_orig.exe`， `ffprobe.exe` 重命名为 `ffprobe_orig.exe`
2. 在 [Releases](https://github.com/ReRokutosei/ffmpeg_hide_banner/releases/tag/v1.0.0) 页面下载已经构建的两个exe，将它们移动到`ffmpeg_orig.exe`同一目录
3. 一切顺利，ffmpeg的bin目录结构应该如下：
     ```sh
     └───bin
             ffmpeg.exe
             ffmpeg_orig.exe
             ffplay.exe
             ffprobe.exe
             ffprobe_orig.exe
     
5. 最后记得将目录配置到环境变量 `PATH` 中

## 🛠️ 从源码构建

```bash
# 构建 ffmpeg launcher
cargo build --release --manifest-path ffmpeg_launcher/Cargo.toml

# 构建 ffprobe launcher
cargo build --release --manifest-path ffprobe_launcher/Cargo.toml
```

## 💡 工作原理

1. 启动器在同目录下查找 `ffmpeg_orig.exe`/`ffprobe_orig.exe`
2. 自动添加 `-hide_banner` 参数
3. 透传用户输入的所有参数
4. 保持原有的返回码

## 📄 协议

本项目采用 MIT 协议。详见 [LICENSE](LICENSE) 文件。
