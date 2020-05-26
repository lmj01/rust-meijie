
fn merge<T: Copy + PartialOrd>(x1: &[T], x2: &[T], y: &mut [T]) {
    assert_eq!(x1.len() + x2.len(), y.len());
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < x1.len() && j < x2.len() {
        if x1[i] < x2[j] {
            y[k] = x1[i];
            k += 1;
            i += 1;
        } else {
            y[k] = x2[j];
            k += 1;
            j += 1;
        }
    }
    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    }
    if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
}

pub fn merge_sort<T: Copy + Ord>(arr: &mut [T]) {
    let n = arr.len();
    let m = n / 2;
    if n <= 1 {
        return;
    }
    merge_sort(&mut arr[0..m]);
    merge_sort(&mut arr[m..n]);

    let mut y: Vec<T> = arr.to_vec();
    merge(&arr[0..m], &arr[m..n], &mut y[..]);
    arr.copy_from_slice(&y);
}

pub fn test() {

    println!("Sort numbers ascending");

    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);

    merge_sort(&mut numbers);
    println!("After: {:?}\n", numbers);

    println!("Sort strings alphabetically");

    let mut names = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", names);

    merge_sort(&mut names);
    println!("After: {:?}\n", names);

}
