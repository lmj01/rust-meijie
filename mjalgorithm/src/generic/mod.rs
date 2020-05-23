//  泛型相关的测试例子

// 关联类型associated types
// 是一个将类型占位符与trait相关联的方式，这样trait的方法签名中就可以使用
// 这些占位符。trait实现者会针对特定的实现这个类型位置的指定相应的具体类型
pub mod watch;
// 默认泛型类型参数
pub mod watch2; 
// 阶乘
pub mod factorial;