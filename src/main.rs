mod hello;

fn main() {
    hello::hello();
    demo_fn();

    let x = 10;
    let y = x;
    println!("{y}");
}

fn demo_fn() {
    println!("Hello from demoFn!");
}