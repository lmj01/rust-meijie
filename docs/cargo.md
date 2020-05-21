

创建一个空的仓库，git clone repository_url

进入目录执行，cargo init 创建一个carte

cargo build

target\debbug\demo.exe

在根目录执行cargo new --lib mjlib
创建一个本地crate供使用
可执行cargo test 进行测试

导入库
本地的
在Cargo.toml文件中dependencies中引入指定库
在main.rs中引入库extern crate mjlib;就可以调用库了
网络的
https://crates.io/
