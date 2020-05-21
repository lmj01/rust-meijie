
// 引入第一个crate进行测试
extern crate mjlib;
// 引入网络库
use random::Source;
// 引入模块
mod mjmath1;
mod mjmath2;

fn main() {
    println!("Hello, world!");

    mjlib::hi("call hi from mjlib crate");

    let mut source = random::default().seed([42,69]); // An instance of the default source.
    println!("Scalar: {:?}", source.read::<f64>());
    println!("Vector: {:?}", source.iter().take(2).collect::<Vec<f64>>());

    let add1 = mjmath1::add(1, 2);
    println!("1 + 2 = {}", add1);

    let add2 = mjmath2::add(1, 2);
    println!("1 + 2 = {}", add2);
}
