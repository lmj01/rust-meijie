

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

## crate

cargo new --lib xxx
创建一个crate
调用时使用use crate::xxx
xxx是当前crate根目录下的mod

注意：生成的src/lib.rs 或src/main.rs是所有入口
如果要外部引用其他模块，需要在这里声明，才能在外部调用

## test
测试按精细度分：
- 函数级，通过#[test]
- 模块级，通过#[cfg(test)]
- 工程级，放于test目录下
- 文档测试

## examples

cargo run --example hello meijie

在根目录下创建examples文件夹，里面每个文件就是一个测试模块