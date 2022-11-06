mod looping;
mod loop_labels;
mod for_while;

fn main() {
    let number: i32 = 5;

    if number > 5 {
        println!("Lucky");
    }
    else if number == 5 {
        println!("Not Lucky");
    }

    let cond = true;

    let no = if cond {5} else {6};

    println!("No = {}", no);

    looping::loops();
    loop_labels::main();
    for_while::main();
}
