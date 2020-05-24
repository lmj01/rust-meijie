use std::ops::Add;

pub fn additems<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

pub fn add_currying<T>(x: T) -> impl Fn(T) -> T 
    where T: num::Num + Copy
{
    move |y: T| x + y
}