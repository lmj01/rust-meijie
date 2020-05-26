
use mjalgorithm::sorts;

fn main() {

    println!("--bubble--");
    sorts::bubble::test();
    println!("--comb of bubble--");
    sorts::comb::test();
    println!("--select--");
    sorts::selection::test();
    println!("--quick--");
    sorts::quick::test();
    println!("--merge--");
    sorts::merge::test();
    println!("--insertion--");
    sorts::insertion::test();
}