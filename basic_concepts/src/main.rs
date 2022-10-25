mod shadowing;
mod datatype;
mod tuple;
mod array;

fn main() {
    let x = 10; // immutable
    println!("x = {x}");

    let mut y = 10; // mutable
    println!("y = {y}");
    y = 20;
    println!("y = {y}");

    shadowing::shadow();
    datatype::types();
    tuple::tup();
    array::arr();
}
