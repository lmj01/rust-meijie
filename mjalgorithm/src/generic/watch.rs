pub trait Watch {
    // add code here
    type Item;
    fn inner(&self) -> Option<Self::Item>;
}

pub struct A {
    pub data: i32,
}

impl Watch for A {
    type Item = i32;
    fn inner(&self) -> Option<Self::Item> {
        Some(self.data)
    }
}

pub struct B {
    pub data : String,
}

impl Watch for B {
    type Item = String;
    fn inner(&self) -> Option<Self::Item> {
        Some(self.data.clone())
    }
}

