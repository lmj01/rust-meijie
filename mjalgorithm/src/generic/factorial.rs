extern crate num;

use std::iter::Product;
use num::{PrimInt, Unsigned};

// find the factorial of n 
pub fn factorial<T>(n: T) -> T 
    where T: PrimInt + Unsigned + Product  
{
    num::range(T::one(), n + T::one()).product()
}