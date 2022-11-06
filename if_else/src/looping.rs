pub fn loops() {
    let mut x: i32 = 1;
    loop {
        x = x + 1;
        if x == 5 {
            println!("Done x = {}", x);
            break;
        }
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}