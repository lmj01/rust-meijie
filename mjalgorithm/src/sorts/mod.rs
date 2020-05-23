
// 代码参考
// https://www.hackertouch.com/data-structures-and-algorithms.html

pub mod bubble;
pub mod uks;

pub fn get5() -> i32 {
    5
}


// type fnType = fn<T: Ord>(&mut [T]) -> ();

// pub fn test_func (fn_ptr: fnType) {

//     println!("Sort numbers ascending");

//     let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
//     println!("Before: {:?}", numbers);

//     fn_ptr(&mut numbers);
//     println!("After: {:?}\n", numbers);

//     println!("Sort strings alphabetically");

//     let mut names = ["beach", "hotel", "airplane", "car", "house", "art"];
//     println!("Before: {:?}", names);

//     fn_ptr(&mut names);
//     println!("After: {:?}\n", names);

// }