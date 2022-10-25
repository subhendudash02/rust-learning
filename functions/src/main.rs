fn main() {
    println!("{} {}", return_type(), return_type2(5));
}

fn return_type() -> f32 {
    return 1.0;
}

fn return_type2(x: i32) -> i32 {
    return x*x;
}