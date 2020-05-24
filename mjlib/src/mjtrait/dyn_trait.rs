
pub trait animal {
    fn print_name(&self);
}

struct cat {
    name: String,
}

struct dog {
    name: String,
}

impl animal for cat {
    fn print_name(&self) {
        println!("{}", self.name);
    }
}

impl animal for dog {
    fn print_name(&self) {
        println!("{}", self.name);
    }
}

// 返回一个对象，但是条件分支是不同的对象类型，这样就会不一致
// pub fn who1(who: i32) -> impl animal {
//     if who == 1 {
//         cat{name: "cat".to_string()}
//     } else {
//         dog{name: "dog".to_string()}
//     }
// }


// 返回一个Box对象，动态分配
pub fn who2(who: i32) -> Box<dyn animal> {
    if who == 1 {
        Box::new(cat{name: "cat".to_string()}) as Box<dyn animal>
    } else {
        Box::new(dog{name: "dog".to_string()}) as Box<dyn animal>
    }
}


