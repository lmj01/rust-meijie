trait Run {

}

trait Eat {

}

#[derive(Debug)]
struct Horse {
    
}

impl Run for Horse {

}

impl Eat  for Horse {

}

// 约束类型对象

fn test1<T: Run + Eat>(x: T) {

}

fn test2<T>(x: T) where T: Run + Eat {

}