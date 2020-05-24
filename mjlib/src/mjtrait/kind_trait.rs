// Disambiguating overlapping traits
// 消除trait歧义

trait TraitOne {
    fn action(&self) {
        println!("action of trait one");
    }
}

trait TraitTwo {
    fn action(&self) {
        println!("action of trait two");
    }
}

struct Person {

}

impl TraitOne for Person {

}

impl TraitTwo for Person {

}

pub fn test() {
    let p = Person{};
    // p.action(); // 编译失败
    <Person as TraitOne>::action(&p);
    <Person as TraitTwo>::action(&p);
}