# Changelog

本项目版本记录遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，版本号遵循 [Semantic Versioning](https://semver.org/lang/zh-CN/)。

## [0.0.5] - 2026-08-18

### Added

- `-o` / `--output` 改为可选，不传时默认写入 `output.jpeg`。
- 压缩完成后对比原图与输出文件体积：成功则打印压缩后大小；若输出比原图更大，给出提示并建议使用 `--size`。

### Changed

- 输出路径处理抽到 `utils::create_output`。

## [0.0.2] - 2026-08-17

### Added

- `--size` / `-s`：按最长边缩放，短边等比缩小；已小于该值则不放大。
- 缩放计算拆成独立函数，并补充单元测试。
- README 补充安装、参数说明和发布流程。

### Changed

- 压缩失败时打印错误信息，不再以 `Result` 方式直接退出。

## [0.0.1] - 2026-08-14

### Added

- 初始 CLI：输入图片并编码为 JPEG。
- `-q` / `--quality`，默认 `80`。
- 基于 `clap` + `image` 的 JPEG 压缩。
