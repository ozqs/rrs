# reader_server

#### 介绍
一个基于 Rocket.rs 的开源文本处理 Web 应用

#### 功能
1. 截取小说片段
2. (Todo: 生成检讨)

#### 软件架构
基于 Rust + rocket.rs

#### 安装教程

1.  Clone it.
2.  `cargo build --release`
3.  `cargo run --bin server`

#### 使用说明

1.  `/` -> indexes 文件，保存文件名
2.  `/<name>/<start>/<seek>` -> 读取 name 小说的从 start 个字符开始 seek 个
3.  若超出范围或书籍不存在，返回404

#### 参与贡献

1.  Fork 本仓库
2.  新建 Feat_xxx 分支
3.  提交代码
4.  新建 Pull Request
