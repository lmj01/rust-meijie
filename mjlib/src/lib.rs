#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// 添加第一个函数用于被调用测试
pub fn hi(name: &str) {
    println!("hi : {}", name);
}