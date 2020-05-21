/**
 * 测试Into的功能
 * 同时接收&str或String作为参数
 * 
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_into() {

        let path = "./Cargo.toml";
        let config = Config::new(path);
        assert_eq!(path, config.get_path());

        let path = "./Cargo.toml";
        let config = Config::new(path.to_string());
        assert_eq!(path, config.get_path())
    }
}
