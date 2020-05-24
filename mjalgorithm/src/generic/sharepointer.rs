use std::rc::Rc;
use std::sync::Arc;

// 定义一个trait
pub trait SharedPointer<T>: Clone {
    fn new(v: T) -> Self;
    // fn get(&self) -> &T;
}

impl<T> SharedPointer<T> for Rc<T> {
    fn new(v: T) -> Self {
        Rc::new(v)
    }
}

impl<T> SharedPointer<T> for Arc<T> {
    fn new(v: T) -> Self {
        Arc::new(v)
    }
}

pub trait Param<T> {
    type Pointer: SharedPointer<T>;
}

pub struct RcParam;
pub struct ArcParam;

impl<T> Param<T> for RcParam {
    type Pointer = Rc<T>;
}

impl<T> Param<T> for ArcParam {
    type Pointer = Arc<T>;
}

pub struct A;
pub struct B;

pub struct Foo<P: Param<A> + Param<B>> {
    a: <P as Param<A>>::Pointer,
    b: <P as Param<B>>::Pointer,
}

impl<P: Param<A> + Param<B>> Foo<P> {
    pub fn new(a: A, b: B) -> Foo<P> {
        Foo {
            a: <P as Param<A>>::Pointer::new(a),
            b: <P as Param<B>>::Pointer::new(b),
        }
    }
}
