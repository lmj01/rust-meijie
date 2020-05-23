use mjalgorithm::generic::watch::{A, B, Watch};
use mjalgorithm::generic::watch2::{A2, B2, Watch2};

use mjalgorithm::generic::factorial::factorial;

fn main() {
    let a = A{data: 10};
    let b = B{data: String::from("B")};
    assert_eq!(Some(10), a.inner());
    assert_eq!(Some(String::from("B")), b.inner());

    let a2 = A2{data: 10};
    let b2 = B2{data: String::from("B")};
    assert_eq!(Some(10), a2.inner());
    assert_eq!(Some(10), a2.info());
    assert_eq!(Some(String::from("B")), b2.inner());
    assert_eq!(Some(String::from("B")), b2.info());

    println!("u8: 3! = {}", factorial(3_u8));
    println!("u16: 3! = {}", factorial(3_u16));
    println!("u32: 3! = {}", factorial(3_u32));
    println!("u64: 3! = {}", factorial(3_u64));
}