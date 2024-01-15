mod structs;

fn add(x: i32, y: i32) -> i32 {
    x + y // or return x + y;
}

fn main() {
    let x: i32 = 20;
    println!("Hello, world!");
    println!("x = {x}");
    println!("3 + 5 = {}", add(3, 5));
    let mut y: i32 = 20;

    if y % 2 == 0 {
        y = y / 2;
        println!("y = {y}");
    }

    let mut i: i32 = 0;

    loop {      // runs until break is given
        i += 1;
        println!("{i}");
        if i > 10 {
            break;
        }
    }

    let (a, b) = (4, 5);

    println!("Initialised in different way: {a}, {b}");

    // blocks

    let bl = {
        println!("a = {a}");
        a - b       // no semi-colons here
    };

    println!("bl = {bl}");

    // shadowing

    let str = 34;
    println!("str = {str}");

    {
        let str = "hello";
        println!("str = {str}");
        let str = false;
        println!("str = {str}");
    }

    // arrays
    let mut arr: [i32; 10] = [0; 10];
    arr[4] = 6;
    println!("arr: {arr:?}");

    // tuple
    let tup: (i32, bool, &str) = (4, false, "subhendu");
    println!("first element: {}", tup.0);
    println!("second element: {}", tup.1);

    let nos = [1, 2, 3, 4, 5];
    
    for i in nos {
        println!("{i}");
    }

    println!("\n");

    for i in 1..10 {
        println!("{i}");
    }

    let inp = 'x';

    match inp {     // similar to switch-case in C++
        'q' => println!("Quitting"),
        'w' | 'a' | 's' | 'd' => println!("moving"),
        '0'..='9' => println!("number"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _ => println!("something else"),
    }

    // shared references

    let a = 5;
    let b = 10;
    let mut r: &i32 = &a;
    println!("Shared reference value: {}", *r);
    r = &20;
    println!("New shared reference value: {}", *r);
    r = &b;
    println!("New shared reference value: {}", *r);

    // exclusive/mutable references -- type: &mut T

    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("Point: {point:?}");

    structs::structs();
}
