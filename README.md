# image-cooking

命令行图片压缩工具：把图片转成 JPEG，可调质量和最长边，用来把照片压小。

## 安装

需要先安装 [Rust](https://rustup.rs)（带 `cargo`）。

从 crates.io 安装：

```bash
cargo install image-cooking
```

或在本仓库目录安装当前源码：

```bash
cargo install --path .
```

装好后任意目录都能用 `image-cooking`。确认：

```bash
image-cooking --help
```

## 使用方法

1. 用终端进入**图片所在的文件夹**（或后面写绝对路径）。
2. 至少提供：**输入文件** 和 `-o` **输出路径**。不写 `-o` 会报错退出。
3. 执行命令。成功会打印 `Image compressed successfully`。

### 最常用

只压质量（默认质量 80），不改尺寸：

```bash
image-cooking photo.jpg -o photo_small.jpeg
```

同时限制最长边（例如最长边不超过 1920 像素，短边按比例缩小）：

```bash
image-cooking photo.jpg -o photo_small.jpeg --size 1920
```

质量和最长边一起用：

```bash
image-cooking photo.jpg -o out.jpeg -q 70 --size 1280
```

短选项写法一样：

```bash
image-cooking photo.jpg -o out.jpeg -q 70 -s 1280
```



### 参数说明


| 参数                | 必填  | 默认值  | 含义                                                         |
| ----------------- | --- | ---- | ---------------------------------------------------------- |
| `<input>`         | 是   | 无    | 输入图片路径。可以是当前目录下的文件名，也可以是绝对路径。支持 JPEG、PNG 等 `image` 能解码的格式。 |
| `-o`, `--output`  | 是   | 无    | 输出文件路径。会**新建或覆盖**这个文件。                                     |
| `-q`, `--quality` | 否   | `80` | JPEG 质量，范围按编码器为 0–100。数字越小体积越小、越糊。                         |
| `--size`          | 否   | 不缩放  | 最长边像素上限。宽图按宽度缩，竖图按高度缩，另一边按比例变。已经小于该值则不放大。                  |


不传 `--size` 时只做 JPEG 重编码，分辨率不变。

### 完整示例

当前目录有 `background.png`，压完写到 `output.jpeg`，质量 90，最长边 600：

```bash
cd ~/Pictures
image-cooking background.png -o output.jpeg -q 90 --size 600
```

输入用绝对路径也可以，不必先 `cd`：

```bash
image-cooking /Users/you/Pictures/photo.jpg -o /Users/you/Pictures/photo_small.jpeg --size 1920
```

在本仓库里调试、不安装全局命令时：

```bash
cargo run -- background.png -o output.jpeg -q 90 --size 600
```

注意：`cargo run` 后面要加 `--`，再写给 `image-cooking` 的参数。

## 使用注意

- **输出始终是 JPEG。** `-o out.png` 写进去的仍是 JPEG 数据，建议输出用 `.jpeg` / `.jpg`。
- **会覆盖已有文件。** `-o` 指向的路径如果已存在，会被直接覆盖。
- 透明 PNG 可能失败（JPEG 不支持透明通道）。
- 体积变小主要靠 `--size`（减像素）和 `-q`（降质量）。大照片优先加 `--size`。



## 查看帮助

```bash
image-cooking --help
image-cooking -V
```

