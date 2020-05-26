pub fn test1() {
    let x = std::ops::Range{ start: 0, end: 10};
    let y = 0..10;
    assert_eq!(x, y);

    println!("range from for in 1..11");
    for i in 1..11 {
        print!(" {}", i);
    }
    print!("\n");

    println!("range from for in 3..");
    for i in 3.. {
        if i==10 {
            break;
        }
        print!(" {}", i);
    }
    print!("\n");

    // println!("range from for in ..4");
    // for i in ..4 {
    //     print!(" {}", i);
    // }
    // print!("\n");

    println!("range from for in 5..=16");
    for i in 5..=16 {
        print!(" {}", i);
    }
    print!("\n");

    println!("range from for in 1..=7");
    for i in 1..=7 {
        print!(" {}", i);
    }
    print!("\n");
}