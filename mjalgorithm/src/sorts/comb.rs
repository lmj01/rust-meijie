
// 是冒泡算法的变形
pub fn comb_sort<T: Ord>(arr: &mut [T]) {
    let mut gap = arr.len() as f64; // initialize gap size
    let shrink = 1.3; // set the gap shrink factor
    let mut sorted = false;

    while sorted == false {
        // update the gap value for a next comb
        gap = gap as f64 / shrink as f64;
        gap = gap.floor();

        if gap <= 1.0 {
            gap = 1.0;
            sorted = true;
        }

        let mut i = 0.0;
        while i + gap < arr.len() as f64 {
            if arr[i as usize] > arr[i as usize + gap as usize] {
                arr.swap(i as usize, i as usize + gap as usize);
                sorted = false;
            }
            i += 1.0;
        }
    }
}

pub fn test() {

    println!("Sort numbers ascending");

    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);

    comb_sort(&mut numbers);
    println!("After: {:?}\n", numbers);

    println!("Sort strings alphabetically");

    let mut names = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", names);

    comb_sort(&mut names);
    println!("After: {:?}\n", names);

}
