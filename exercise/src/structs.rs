const CONST_NO: i32 = 4;
static STATIC_NO: i32 = 5;

// named structs
struct Person {
    name: String,
    age: i32,
}

// tuple structs
struct Person2(String, i32);

// enums
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,               // simple variant
    Run (Direction),    // tuple variant
    Teleport {
        x: i32,
        y: i32,
    }                   // struct variant
}

fn describe(person: &Person) {
    println!("{} is {} years old.", person.name, person.age);
}

pub fn structs() {
    let mut subu = Person {
        name: String::from("Subhendu"),
        age: 21
    };

    describe(&subu);
    subu.age = 22;
    describe(&subu);

    let mut new = Person {
        name: String::from("anonymous"),
        age: 30
    };

    describe(&new);

    let new2 = Person {
        name: String::from("new2"),
        ..new
    };

    describe(&new2);

    let p = Person2(String::from("subu"), 21);
    println!("({}, {})", p.0, p.1);

    let m: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("On this turn: {:?}", m);

    println!("Const value: {CONST_NO}");
    println!("Static value: {STATIC_NO}");
}