pub trait Watch2<Inner=String> {
    // add code here
    type Item;
    fn inner(&self) -> Option<Self::Item>;
    fn info(&self) -> Option<Inner>;
}

pub struct A2 {
    pub data: i32,
}

impl Watch2<i32> for A2 {
    type Item = i32;
    fn inner(&self) -> Option<Self::Item> {
        Some(self.data)
    }
    fn info(&self) -> Option<i32> {
        println!("A inner is {}", self.data);
        Some(self.data)
    }
}

pub struct B2 {
    pub data : String,
}

impl Watch2 for B2 {
    type Item = String;
    fn inner(&self) -> Option<Self::Item> {
        Some(self.data.clone())
    }
    fn info(&self) -> Option<String> {
        println!("B inner is {}", self.data);
        Some(self.data.clone())
    }
}

