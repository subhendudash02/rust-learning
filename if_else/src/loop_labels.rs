pub fn main() {
    let mut o: i32 = 0;
    'outer : loop {
        println!("o = {}", o);
        o += 1;

        if o == 5 {
            println!("Over");
            break 'outer;
        }
    }
}