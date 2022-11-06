pub fn main() {
    let mut x: i32 = 1;
    while x < 10 {
        println!("x = {}", x);
        x += 1;
    }

    let a = [1, 2, 3, 4, 5];

    for i in a {
        println!("{i}");
    }

    for i in 1..5 {
        println!("{i}");
    }

    for i in (1..5).rev() {
        println!("{i}");
    }
}