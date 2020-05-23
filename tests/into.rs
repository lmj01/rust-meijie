/**
 * Trait 
 * std::convert::From 允许一种类型定义：怎么根据另一种类型生成自己。是一种类型转换的简单机制
 * std::convert::Into 是From逆过程，即类型实现了From，同时会获得Into。Into需要指定转换的类型
 * 
 *
 * 测试Into的功能
 * 同时接收&str或String作为参数
 *  
 */

// impl<T, U> Into<U> for T where U: From<T>
// {
//     fn into(self) -> U {
//         U::from(self)
//     }
// }

// impl From<&str> for String {
//     #[inline]
//     fn from(s: &str) -> String {
//         s.to_owned()
//     }
// }


struct Config {
    path: String
}

impl Config {

    // 外部传入&str时，需要手动转为String
    // 测试第一段不能通过
    // fn new(path: String) -> Config {
    //     Config { path }
    // }

    // 测试都能通过
    fn new<T: Into<String>>(path: T) -> Config {
        Config { path: path.into() }
    }


    fn get_path(self) -> String {
        self.path
    }
}

fn is_hello<T: Into<Vec<u8>>>(s: T) {
    let bytes = b"hello".to_vec();
    assert_eq!(bytes, s.into());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config() {

        let path = "./Cargo.toml";
        let config = Config::new(path);
        assert_eq!(path, config.get_path());

        let path = "./Cargo.toml";
        let config = Config::new(path.to_string());
        assert_eq!(path, config.get_path());

    }

    #[test]
    fn hello() {
        let s1 = "hello";
        is_hello(s1);

        let s2 = "hello".to_string();
        is_hello(s2);
    }
}
