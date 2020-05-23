//! The `adder` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
// ! ```
//! assert_eq!(4, adder::add_two(2));
//! ```

/// This function adds two to its argument.
///
/// # Examples
///
// / ```
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```

// extern crate test; // 引入测试模块

pub mod mjsort;
// use mjsort::uks;

pub fn add_two(a: i32) -> i32 {
   a + 2
}

#[cfg(test)]
mod tests {
   use super::*;
//    use test::Bencher;

   #[test]
   fn it_works() {
      assert_eq!(4, add_two(2));
   }

   #[test]
   fn is_five() {
      assert_eq!(5, mjsort::get5());
      assert_eq!(5, mjsort::uks::get5());
   }

    // #[bench]
    // fn bench_add_two(b: &mut test::Bencher) {
    //     b.iter(|| add_two(2))
    // }
}

