
pub fn select_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
    }
}

pub fn test() {

    println!("Sort numbers ascending");

    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);

    select_sort(&mut numbers);
    println!("After: {:?}\n", numbers);

    println!("Sort strings alphabetically");

    let mut names = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", names);

    select_sort(&mut names);
    println!("After: {:?}\n", names);

}
