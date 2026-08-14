## 项目介绍

image-cooking 是一个用于压缩图片的 CLI 工具。

## 使用方法
我们可以终端进入一个目录下，然后执行以下命令：
```bash
image-cooking <input> -o <output> -q <quality>
```

## 参数说明
- input: 输入图片路径，可以是相对路径或绝对路径
- output: 输出图片路径
- quality: 压缩质量

## 发布流程

### 登陆
```bash
cargo login
```

### 打包检查
```bash
cargo publish --allow-dirty
```


### 发布
```bash
cargo publish
```
