
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub fn bubble_test() {

    println!("Sort numbers ascending");

    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);

    bubble_sort(&mut numbers);
    println!("After: {:?}\n", numbers);

    println!("Sort strings alphabetically");

    let mut names = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", names);

    bubble_sort(&mut names);
    println!("After: {:?}\n", names);

}
