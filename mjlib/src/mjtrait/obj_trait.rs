pub trait Run {

}

struct Human;
struct Cat;

impl Run for Human {

}

impl Run for Cat {

}

fn demo(x: Vec<Box<dyn Run>>) {

}

pub fn test() {

    // trait object 本质是指针，它可以指向不同的类型

    let mut v: Vec<Box<dyn Run>> = vec![];
    v.push(Box::new(Human{}));
    v.push(Box::new(Cat{}));
    demo(v);
}