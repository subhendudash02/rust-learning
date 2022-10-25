pub fn arr() {
    let x: [i32; 3] = [1, 2, 3];
    println!("{} {} {}", x[0], x[1], x[2]);

    let y = [5; 3];
    println!("{}", y[1]);
}